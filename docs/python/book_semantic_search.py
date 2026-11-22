#!/usr/bin/env python3
"""基于 Zvec + fastembed 的本书语义检索工具。

用法:
    python book_semantic_search.py index [--rebuild] [--model bge|qwen3] [--limit N]
    python book_semantic_search.py "TLS 握手过程" [-k 5] [--model bge|qwen3] [--limit N]

模型:
    bge    BAAI/bge-small-zh-v1.5，512 维，首次使用会下载约 90 MB
    qwen3  Qwen3-Embedding-0.6B 量化版，1024 维，需先放在 local/qwen3-embedding-0.6b

--limit N 只索引前 N 个章节文件，用于小范围对比实验；查询时需带相同的 --limit。

无法访问 Hugging Face 时，先设置镜像再运行:
    cmd:        set HF_ENDPOINT=https://hf-mirror.com
    PowerShell: $env:HF_ENDPOINT = "https://hf-mirror.com"
"""

from __future__ import annotations

import argparse
import re
import shutil
import sys
import time
from pathlib import Path
from typing import Iterable

import zvec

PYTHON_DIR = Path(__file__).resolve().parent
SRC_DIR = PYTHON_DIR.parent
REPO_ROOT = SRC_DIR.parent
LEGACY_INDEX_DIR = REPO_ROOT / ".book_index"

MODEL_PROFILES = {
    "bge": {
        "model_name": "BAAI/bge-small-zh-v1.5",
        "dimension": 512,
        "query_instruction": None,
        "local_path": None,
    },
    "qwen3": {
        "model_name": "Qwen/Qwen3-Embedding-0.6B-Q",
        "dimension": 1024,
        "query_instruction": (
            "Given a web search query, retrieve relevant passages that answer the query"
        ),
        "local_path": REPO_ROOT / "local" / "qwen3-embedding-0.6b",
    },
}
DEFAULT_MODEL = "bge"

COLLECTION_NAME = "book_chapters"
CHUNK_SIZE = 500
CHUNK_OVERLAP = 100
MIN_CHUNK_CHARS = 30
INSERT_BATCH = 64

FENCE_RE = re.compile(r"```.*?```", re.S)
COMMENT_RE = re.compile(r"<!--.*?-->", re.S)
HEADING_RE = re.compile(r"^#{1,6}\s+(.*)$")
TITLE_RE = re.compile(r"^#\s+(.*)$", re.M)


def _chapter_sort_key(path: Path) -> tuple[int, int, str]:
    match = re.match(r"chapter_(\d+)", path.stem)
    if match:
        return (0, int(match.group(1)), path.name)
    return (1, 0, path.name)


def chapter_files(limit: int | None = None) -> list[Path]:
    files = sorted(
        (path for path in SRC_DIR.glob("*.md") if path.name != "SUMMARY.md"),
        key=_chapter_sort_key,
    )
    return files[:limit] if limit is not None else files


def index_dir_for(model_key: str, limit: int | None) -> Path:
    if model_key == DEFAULT_MODEL and limit is None:
        return LEGACY_INDEX_DIR
    name = f".book_index_{model_key}"
    if limit is not None:
        name += f"_top{limit}"
    return REPO_ROOT / name


def clean_markdown(raw: str) -> str:
    return COMMENT_RE.sub("", FENCE_RE.sub("", raw))


def chapter_title(raw: str, fallback: str) -> str:
    match = TITLE_RE.search(raw)
    return match.group(1).strip() if match else fallback


def split_sections(text: str) -> Iterable[tuple[str, str]]:
    heading = ""
    lines: list[str] = []
    for line in text.splitlines():
        match = HEADING_RE.match(line)
        if match:
            body = "\n".join(lines).strip()
            if body:
                yield heading, body
            heading = match.group(1).strip()
            lines = []
        else:
            lines.append(line)
    body = "\n".join(lines).strip()
    if body:
        yield heading, body


def chunk_text(text: str) -> Iterable[str]:
    if len(text) <= CHUNK_SIZE:
        yield text
        return
    start = 0
    while start < len(text):
        end = min(start + CHUNK_SIZE, len(text))
        yield text[start:end]
        if end == len(text):
            return
        start = end - CHUNK_OVERLAP


def build_chunks(limit: int | None = None) -> tuple[list[tuple[str, str, str]], int]:
    files = chapter_files(limit)
    chunks: list[tuple[str, str, str]] = []
    for path in files:
        raw = path.read_text(encoding="utf-8")
        title = chapter_title(raw, path.stem)
        for heading, body in split_sections(clean_markdown(raw)):
            prefix = f"《{title}》{heading}\n" if heading else f"《{title}》\n"
            for piece in chunk_text(body):
                if len(piece.strip()) >= MIN_CHUNK_CHARS:
                    chunks.append((path.stem, heading, prefix + piece))
    return chunks, len(files)


def collection_schema(dimension: int) -> zvec.CollectionSchema:
    return zvec.CollectionSchema(
        name=COLLECTION_NAME,
        fields=[
            zvec.FieldSchema(name="chapter", data_type=zvec.DataType.STRING),
            zvec.FieldSchema(name="heading", data_type=zvec.DataType.STRING),
            zvec.FieldSchema(name="text", data_type=zvec.DataType.STRING),
        ],
        vectors=[
            zvec.VectorSchema(
                name="embedding",
                data_type=zvec.DataType.VECTOR_FP32,
                dimension=dimension,
                index_param=zvec.HnswIndexParam(metric_type=zvec.MetricType.COSINE),
            )
        ],
    )


def load_model(profile: dict):
    try:
        from fastembed import TextEmbedding
    except ImportError:
        sys.exit("缺少 fastembed，请先运行: python -m pip install fastembed")

    kwargs = {"model_name": profile["model_name"]}
    local_path = profile.get("local_path")
    if local_path is not None:
        local_path = Path(local_path)
        if not local_path.exists():
            sys.exit(f"本地模型目录不存在: {local_path}")
        kwargs["specific_model_path"] = str(local_path)

    try:
        return TextEmbedding(**kwargs)
    except Exception as exc:
        sys.exit(
            f"加载嵌入模型失败: {exc}\n"
            "若无法访问 Hugging Face，可先设置 HF_ENDPOINT=https://hf-mirror.com"
        )


def cmd_index(rebuild: bool, model_key: str, limit: int | None) -> None:
    profile = MODEL_PROFILES[model_key]
    index_dir = index_dir_for(model_key, limit)

    if index_dir.exists():
        if not rebuild:
            print(f"索引已存在: {index_dir}")
            print("如需重建，请运行: python book_semantic_search.py index --rebuild")
            return
        shutil.rmtree(index_dir)

    chunks, file_count = build_chunks(limit)
    if not chunks:
        sys.exit(f"在 {SRC_DIR} 下没有找到可索引的 Markdown 章节")

    print(f"模型: {profile['model_name']}（{profile['dimension']} 维）")
    print(f"章节: {file_count} 个，片段: {len(chunks)} 个")

    model = load_model(profile)
    collection = zvec.create_and_open(
        str(index_dir), collection_schema(profile["dimension"])
    )

    started = time.time()
    batch: list[zvec.Doc] = []
    inserted = 0
    for index, ((chapter, heading, text), vector) in enumerate(
        zip(chunks, model.embed([chunk[2] for chunk in chunks], batch_size=16))
    ):
        batch.append(
            zvec.Doc(
                id=f"{chapter}#{index}",
                vectors={"embedding": vector.tolist()},
                fields={"chapter": chapter, "heading": heading, "text": text},
            )
        )
        if len(batch) >= INSERT_BATCH:
            collection.insert(batch)
            inserted += len(batch)
            batch.clear()
            print(f"\r已写入 {inserted}/{len(chunks)} 个片段", end="", flush=True)

    if batch:
        collection.insert(batch)
        inserted += len(batch)

    collection.flush()
    collection.optimize()
    elapsed = time.time() - started

    print(f"\r索引完成: {inserted} 个片段，来自 {file_count} 个章节")
    print(f"索引位置: {index_dir}，用时 {elapsed:.1f} 秒")


def cmd_query(text: str, topk: int, model_key: str, limit: int | None) -> None:
    profile = MODEL_PROFILES[model_key]
    index_dir = index_dir_for(model_key, limit)

    if not index_dir.exists():
        print(f"索引不存在: {index_dir}")
        print("请先运行: python book_semantic_search.py index")
        return

    model = load_model(profile)
    query = text
    if profile["query_instruction"]:
        query = f"Instruct: {profile['query_instruction']}\nQuery:{text}"
    query_vector = next(iter(model.query_embed(query)))

    collection = zvec.open(str(index_dir))
    results = collection.query(
        zvec.Query(field_name="embedding", vector=query_vector.tolist()),
        topk=topk,
        output_fields=["chapter", "heading", "text"],
    )

    print(f"查询: {text}")
    print(f"模型: {profile['model_name']}（{profile['dimension']} 维）")
    for rank, doc in enumerate(results, 1):
        fields = doc.fields or {}
        snippet = " ".join(str(fields.get("text", "")).split())
        if len(snippet) > 150:
            snippet = snippet[:150] + "…"
        distance = doc.score if doc.score is not None else float("nan")
        heading = fields.get("heading") or "（无小节标题）"
        print(
            f"\n[{rank}] 相似度={1.0 - distance:.4f}  "
            f"{fields.get('chapter')} / {heading}"
        )
        print(f"    {snippet}")


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(description="用 Zvec 对本书章节做语义检索")
    subparsers = parser.add_subparsers(dest="command")

    index_parser = subparsers.add_parser("index", help="建立或重建索引")
    index_parser.add_argument(
        "--rebuild", action="store_true", help="删除旧索引并重新建立"
    )
    index_parser.add_argument(
        "--model",
        choices=list(MODEL_PROFILES),
        default=DEFAULT_MODEL,
        help=f"嵌入模型（默认 {DEFAULT_MODEL}）",
    )
    index_parser.add_argument(
        "--limit", type=int, default=None, help="只索引前 N 个章节文件（对比实验用）"
    )

    query_parser = subparsers.add_parser("query", help="检索相关段落")
    query_parser.add_argument("text", help="查询内容")
    query_parser.add_argument(
        "-k", "--topk", type=int, default=5, help="返回条数（默认 5）"
    )
    query_parser.add_argument(
        "--model",
        choices=list(MODEL_PROFILES),
        default=DEFAULT_MODEL,
        help=f"嵌入模型（默认 {DEFAULT_MODEL}）",
    )
    query_parser.add_argument(
        "--limit", type=int, default=None, help="查询对比实验索引时必须与建索引时一致"
    )
    return parser


def main(argv: list[str] | None = None) -> None:
    argv = list(sys.argv[1:] if argv is None else argv)
    if argv and argv[0] not in {"index", "query", "-h", "--help"}:
        argv.insert(0, "query")

    parser = build_parser()
    args = parser.parse_args(argv)

    if args.command == "index":
        cmd_index(args.rebuild, args.model, args.limit)
    elif args.command == "query":
        cmd_query(args.text, args.topk, args.model, args.limit)
    else:
        parser.print_help()


if __name__ == "__main__":
    main()
