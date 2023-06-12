import matplotlib.pyplot as plt
import json
import numpy as np

folder_name = "50k_pure_hervor"
f = open(f'../_data_archive/instances/{folder_name}/training_stats.txt', 'r')

text = f.read()[1:-1].split('][')
chaser_avg = []
hervor_avg = []

print(len(text))

for x in text:
    sp = x.split('},{')
    hervor = json.loads(sp[0] + "}")
    chaser = json.loads("{" + sp[1])
    hervor_avg.append(hervor["avg_fitness"])
    chaser_avg.append(chaser["avg_fitness"])

window = 100
average = []
for ind in range(len(hervor_avg) - window + 1):
    average.append(np.average(hervor_avg[ind:ind+window]))

plt.plot(range(len(hervor_avg)), hervor_avg)
plt.plot(range(len(hervor_avg) - window + 1), average)
plt.title("Hervor average fitness")
plt.show()

window = 100
average = []
for ind in range(len(chaser_avg) - window + 1):
    average.append(np.average(chaser_avg[ind:ind+window]))

plt.plot(range(len(chaser_avg)), chaser_avg)
plt.plot(range(len(chaser_avg) - window + 1), average)
plt.title("Chaser average fitness")
plt.show()