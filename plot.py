import matplotlib.pyplot as plt
import glob
import numpy as np

def graph_scatter(x,y):
	# labels = []
	# for x_val, y_val in zip(x,y):
	# 	#labels.append(str(x_val) + "," + str(y_val))
	# 	labels.append(str(y_val))

	plt.scatter(x,y)

	plt.title("Data movement distance (DMD) of Quicksort")
	plt.xlabel("Array size")
	plt.ylabel("DMD")
	#plt.yscale("log")

	# for i, txt in enumerate(labels):
	# 	plt.annotate(txt, (x[i], y[i]))

	plt.show()

# def collect_x():
# 	res = []
# 	path = "/u/agoldfa7/research/tracker/target/debug/"
# 	for file in sorted(glob.glob(path+"*.txt")):
# 		cur = open(file)
# 		line_arr = cur.readline().split(" ")
# 		if len(line_arr) > 1:
# 			num = line_arr[1].replace("\n", "")
# 			res.append(int(num))
# 		cur.close()
# 	return res

# def collect_y():
# 	res = []
# 	path = "/u/agoldfa7/research/tracker/target/debug/"
# 	for file in sorted(glob.glob(path+"*.txt")):
# 		cur = open(file)
# 		lines = cur.readlines()
# 		if len(lines) > 2:
# 			#print(lines)
# 			line_arr = lines[2].split(" ")
# 		if len(line_arr) > 1:
# 			num = line_arr[1].replace("\n", "")
# 			res.append(float(num))
# 		cur.close()
# 	return res

def dmd_scatter():
	x = collect_x()
	y = collect_y()
	print(x,y)
	graph_scatter(x,y)

def collect_data():
	ab = []
	c = []
	temp = []
	path = "/u/agoldfa7/research/tracker/mm_data/"
	for file in sorted(glob.glob(path+"*.txt")):
		cur = open(file)
		lines = cur.readlines()
		
		ab.append(float(lines[2].split(": ")[1].replace("\n", "")))
		c.append(float(lines[3].split(": ")[1].replace("\n", "")))
		temp.append(float(lines[4].split(": ")[1].replace("\n", ""))) 

		# ab.append(lines[2].split(": ")[1].replace("\n", ""))
		# c.append(lines[3].split(": ")[1].replace("\n", ""))
		# temp.append(lines[4].split(": ")[1].replace("\n", ""))
		cur.close()
	ab.sort()
	c.sort()
	temp.sort()
	return (ab,c,temp)

def stacked_bar():
	labels = ['2x2', '4x4', '8x8', '16x16']
	(ab,c,temp) = collect_data()

	ab_arr = np.array(ab)
	c_arr = np.array(c)
	t_arr = np.array(temp)

	width = 0.35       # the width of the bars: can also be len(x) sequence

	fig, ax = plt.subplots()

	ax.bar(labels, ab_arr, width, label='A and B')
	ax.bar(labels, c_arr, width, bottom=ab_arr, label='C')
	ax.bar(labels, t_arr, width, bottom=ab_arr+c_arr, label='temp')
		

	ax.set_ylabel('DMD')
	ax.set_xlabel('Matrix dimensions')
	ax.set_title('DMD of recursive mm')
	ax.legend()

	plt.yscale("log")
	plt.show()

def main():
	stacked_bar()
	#print(collect_data())

if __name__ == "__main__":
	main()
