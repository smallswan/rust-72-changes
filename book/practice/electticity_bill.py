import pandas as pd
import numpy as np
from pandas import date_range
import matplotlib.pyplot as plt

file_path = "AB.xlsx"
# engine="openpyxl",
#sheet_names = ["2016年","2017年","2018年","2019年","2020年","2021年","2022年"]
sheet_names = ["2016年","2017年","2018年","2019年","2020年","2021年","2022年","2023年","2024年","2025年"]
df = pd.read_excel(file_path, sheet_name=sheet_names, header=0, index_col=0, na_values=['', 'NA', 'NULL'])
sheet_names = pd.ExcelFile("AB.xlsx").sheet_names

print(sheet_names)
# 获取所有列名
# columns = df.columns
# print(columns)

outer_merge = pd.DataFrame()

def read_multiple_sheets(file_path, sheets=None):
    """
    读取Excel文件中的多个工作表

    参数:
        file_path (str): Excel文件路径
        sheets: 要读取的工作表，可以是名称、索引或列表，默认为None（读取所有工作表）

    返回:
        dict: 键为工作表名称，值为对应的DataFrame
    """
    # 读取指定的工作表，返回字典
    excel_data = pd.read_excel(file_path, sheet_name=sheets)

    # 如果只读取了一个工作表，会返回DataFrame而不是字典，这里统一转为字典
    if not isinstance(excel_data, dict):
        # 获取该工作表的名称
        sheet_names = pd.ExcelFile(file_path).sheet_names
        if sheets is None:
            sheet_name = sheet_names[0]
        elif isinstance(sheets, int):
            sheet_name = sheet_names[sheets]
        else:
            sheet_name = sheets

        excel_data = {sheet_name: excel_data}

    return excel_data

#'上年余额',
column_names = ['上年余额','1月应交', '1月已交', '2月应交', '2月已交', '3月应交', '3月已交', '4月应交',
       '4月已交', '5月应交', '5月已交', '6月应交', '6月已交', '7月应交', '7月已交', '8月应交', '8月已交',
       '9月应交', '9月已交', '10月应交', '10月已交', '11月应交', '11月已交', '12月应交', '12月已交']

# 房号索引
index = ['A1801','A1802','A1803','A1804','A1701','A1702','A1703','A1704','A1601','A1602','A1603','A1604','A1501','A1502','A1503','A1504','A1401','A1402','A1403','A1404','A1301','A1302','A1303','A1304','A1201','A1202','A1203','A1204','A1101','A1102','A1103','A1104','A1001','A1002','A1003','A1004','A901','A902','A903','A904','A801','A802','A803','A804','A701','A702','A703','A704','A601','A602','A603','A604','A501','A502','A503','A504','A401','A402','A403','A404','A301','A302','A303','A304','李春园','广电','B1801','B1802','B1803','B1804','B1701','B1702','B1703','B1704','B1601','B1602','B1603','B1604','B1501','B1502','B1503','B1504','B1401','B1402','B1403','B1404','B1301','B1302','B1303','B1304','B1201','B1202','B1203','B1204','B1101','B1102','B1103','B1104','B1001','B1002','B1003','B1004','B901','B902','B903','B904','B801','B802','B803','B804','B701','B702','B703','B704','B601','B602','B603','B604','B501','B502','B503','B504','B401','B402','B403','B404','B302','B303','B304']
print("AB栋房号数量:", len(index))
all_sheets = read_multiple_sheets(file_path)

merge_df = pd.DataFrame({},index=index)
merge_payable = pd.DataFrame(index=index)
merge_received = pd.DataFrame(index=index)

B402_Payable = np.array([])
B402_Received = np.array([])
print(f"读取了 {len(all_sheets)} 个工作表:")
for sheet_name, df in all_sheets.items():
    if sheet_name == "Sheet1":
        break
    df1 = df.set_index('房号')
    #df1.columns = column_names
    #merge_df.index = df1.index
    print(f"\n工作表: {sheet_name}")
    print(f"列名: {df1.columns}")
    merge_df = pd.concat([merge_df,df1.iloc[:,1:]],axis=1,join='outer')
    merge_payable = pd.concat([merge_payable,df1.iloc[:,1::2]],axis=1,join='outer')
    merge_received = pd.concat([merge_received,df1.iloc[:,2::2]],axis=1,join='outer')
    #merge_df.columns = sheet_names
    #print(df1.values[:,1::2])
    #print(df1.values[:, 2::2])
    #print(df1.values[-6,1::2])
    B402_Payable = np.append(B402_Payable,df1.values[-6,1::2])
    B402_Received = np.append(B402_Received,df1.values[-6,2::2])
    if df1.columns.size == 25:
        payable = pd.DataFrame(df1.values[:,1::2], columns=column_names[1::2], index=df1.index)
        print(payable)
        print(payable.iloc[-6])
        received = pd.DataFrame(df1.values[:,2::2], columns=column_names[2::2], index=df1.index)
        print(received)


    #print(f"数据行数: {len(df1)}")
    #print(f"列名: {df1.columns}")
    #print(f"索引: {df1.index}")
    #print("前5行数据:")
    #print(df1.head())
    #print(df1.iloc[0])
    #merge_df.merge(df, how='outer')
    #outer_merge = pd.merge(merge_df, df, on='房号', how='outer')

# 将数据写到新的Excel文件中
merge_df.to_excel("merged_data.xlsx", index=True)
merge_payable.to_excel("payable.xlsx", index=True)
merge_received.to_excel("received.xlsx", index=True)

