import matplotlib.pyplot as plt
import glob

def graph(x,y):
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

def collect_x():
	res = []
	path = "/u/agoldfa7/research/tracker/target/debug/"
	for file in sorted(glob.glob(path+"*.txt")):
		cur = open(file)
		line_arr = cur.readline().split(" ")
		if len(line_arr) > 1:
			num = line_arr[1].replace("\n", "")
			res.append(int(num))
		cur.close()
	return res

def collect_y():
	res = []
	path = "/u/agoldfa7/research/tracker/target/debug/"
	for file in sorted(glob.glob(path+"*.txt")):
		cur = open(file)
		lines = cur.readlines()
		if len(lines) > 2:
			#print(lines)
			line_arr = lines[2].split(" ")
		if len(line_arr) > 1:
			num = line_arr[1].replace("\n", "")
			res.append(float(num))
		cur.close()
	return res

def main():
	x = collect_x()
	y = collect_y()
	print(x,y)
	graph(x,y)

if __name__ == "__main__":
	main()
