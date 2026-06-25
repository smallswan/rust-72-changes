# 国密示例 sm-crypto
## SM3
from sm_crypto import sm2, sm3, sm4
sm3_hash = sm3.sm3_hash(bytearray(b"123456"))
print(sm3_hash.upper())
## 读取文件并计算sm3

from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.backends import default_backend
import os


def calculate_sm3_cryptography(file_path):
    """
    使用 cryptography 库计算文件的 SM3 哈希值
    """
    try:
        # 创建 SM3 哈希对象
        digest = hashes.Hash(hashes.SM3(), backend=default_backend())

        # 读取文件并更新哈希
        with open(file_path, 'rb') as f:
            while True:
                chunk = f.read(4096)
                if not chunk:
                    break
                digest.update(chunk)

        # 获取最终哈希值
        hash_bytes = digest.finalize()
        return hash_bytes.hex()

    except FileNotFoundError:
        return f"错误：文件 {file_path} 未找到"
    except Exception as e:
        return f"错误：{str(e)}"


# 使用示例
if __name__ == "__main__":
    file_path = "../alipay.jpg"
    result = calculate_sm3_cryptography(file_path)
    print(f"SM3 哈希值: {result}")


