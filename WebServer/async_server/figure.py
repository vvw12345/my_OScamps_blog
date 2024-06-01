import matplotlib.pyplot as plt
import pandas as pd
import numpy as np

thread_result_path = './results/thread_results.csv'
async_result_path = './results/async_results.csv'
output_img = './results/performance_comparison.png'

# 读取两个 CSV 文件
data_thread = pd.read_csv(thread_result_path)
data_async = pd.read_csv(async_result_path)

# 为了区分两种类型的数据，我们添加一个新列 'server_type'
data_thread['server_type'] = 'Thread-based Server'
data_async['server_type'] = 'Async Server'

# 合并数据集
combined_data = pd.concat([data_thread, data_async])

# 去除异常点
Q1 = combined_data['total_duration'].quantile(0.25)
Q3 = combined_data['total_duration'].quantile(0.75)
IQR = Q3 - Q1
lower_bound = Q1 - 1.5 * IQR
upper_bound = Q3 + 1.5 * IQR

# 去除异常点
combined_data_clean = combined_data[(combined_data['total_duration'] >= lower_bound) &
                                    (combined_data['total_duration'] <= upper_bound)]

# 开始绘图
plt.figure(figsize=(10, 5))

# 根据 'server_type' 区分数据并分别绘制趋势线
for _, group in combined_data_clean.groupby('server_type'):
    coefficients = np.polyfit(group['client_count'], group['total_duration'], deg=1)  # 线性拟合
    function = np.poly1d(coefficients)
    plt.plot(group['client_count'], function(group['client_count']), 
             label=group['server_type'].iloc[0])

# 添加标题和标签
plt.title('Client Count vs Total Duration (Thread vs Async)')
plt.xlabel('Number of Clients')
plt.ylabel('Total Duration (seconds)')

# 添加图例
plt.legend()

# 显示网格
plt.grid(True)

# 保存图像到文件
plt.savefig(output_img, format='png', dpi=300)

# 显示图像
plt.show()