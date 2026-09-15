# -*- coding: utf-8 -*-
"""解析《AB栋住户电梯维护费登记表.xlsx》并写入 electricity_data.db 的 elevator_records 表。

表结构（参考 electricity_records）：
    elevator_records(id, record_date 'YYYY-MM', room, owner, paid REAL)
每条记录表示某房号在某年月实际缴纳的电梯维护费（已交）。
"""
import pandas as pd
import sqlite3
import io, sys, re

sys.stdout = io.TextIOWrapper(sys.stdout.buffer, encoding='utf-8')

FILE = 'AB栋住户电梯维护费登记表.xlsx'
DB = 'electricity_data.db'

xl = pd.ExcelFile(FILE)
month_cols = [f'{m}月已交' for m in range(1, 13)]

rows = []  # (record_date, room, owner, paid)
for sheet in xl.sheet_names:
    year_match = re.search(r'(\d{4})', sheet)
    if not year_match:
        continue
    year = int(year_match.group(1))
    df = pd.read_excel(FILE, sheet_name=sheet, header=2)
    df.columns = [str(c).strip() for c in df.columns]
    if '房号' not in df.columns:
        continue
    body = df[df['房号'].astype(str).str.match(r'^[AB]\d', na=False)]
    for _, r in body.iterrows():
        room = str(r['房号']).strip()
        owner = r.get('原户主')
        owner = None if pd.isna(owner) else str(owner).strip()
        for m in range(1, 13):
            col = f'{m}月已交'
            if col not in df.columns:
                continue
            v = r[col]
            if pd.isna(v):
                continue
            try:
                fv = float(v)
            except (ValueError, TypeError):
                continue
            if fv == 0:
                continue
            record_date = f'{year}-{m:02d}'
            rows.append((record_date, room, owner, round(fv, 2)))

print(f'解析得到 {len(rows)} 条缴费记录')

conn = sqlite3.connect(DB)
cur = conn.cursor()
cur.execute('DROP TABLE IF EXISTS elevator_records')
cur.execute('''
CREATE TABLE elevator_records (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    record_date TEXT NOT NULL,
    room TEXT NOT NULL,
    owner TEXT,
    paid REAL
)
''')
cur.execute('CREATE INDEX idx_elev_date ON elevator_records(record_date)')
cur.execute('CREATE INDEX idx_elev_room ON elevator_records(room)')
cur.executemany(
    'INSERT INTO elevator_records (record_date, room, owner, paid) VALUES (?, ?, ?, ?)',
    rows,
)
conn.commit()

# 校验
cur.execute('SELECT COUNT(*), COUNT(DISTINCT room), MIN(record_date), MAX(record_date), ROUND(SUM(paid),2) FROM elevator_records')
print('elevator_records:', cur.fetchone())
cur.execute('SELECT record_date, room, owner, paid FROM elevator_records ORDER BY record_date, room LIMIT 5')
for row in cur.fetchall():
    print(row)
conn.close()
print('done')
