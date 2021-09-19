import matplotlib.pyplot as plt
import glob
import numpy as np
import json


def collect_freq():
	ab = {}
	c = {}
	t = {}
	path = "/u/agoldfa7/research/tracker/"
	for file in sorted(glob.glob(path+"*.txt")):
		cur = open(file)
		
		a_line = cur.readline().split(",")
		for line in a_line:
			line = line.replace("{"," ").replace("\n","").replace("}","")
			tmp = line.split(":")
			if len(tmp) == 2:
				ab[int(tmp[0])] = int(tmp[1])
		
		c_line = cur.readline().split(",")
		for line in c_line:
			line = line.replace("{"," ").replace("\n","").replace("}","")
			tmp = line.split(":")
			if len(tmp) == 2:
				c[int(tmp[0])] = int(tmp[1])

		t_line = cur.readline().split(",")
		for line in t_line:
			line = line.replace("{"," ").replace("\n","").replace("}","")
			tmp = line.split(":")
			if len(tmp) == 2:
				t[int(tmp[0])] = int(tmp[1])

		cur.close()

	return (ab,c,t)

def graph_scatter(d,title,xlab,ylab,filename):
	# labels = []
	# for x_val, y_val in zip(x,y):
	# 	#labels.append(str(x_val) + "," + str(y_val))
	# 	labels.append(str(y_val))

	for k,v in sorted(d.items()):
		plt.scatter(k,v)

	plt.title(title)
	plt.xlabel(xlab)
	plt.ylabel(ylab)
	#plt.yscale("log")

	# for i, txt in enumerate(labels):
	# 	plt.annotate(txt, (x[i], y[i]))

	plt.savefig("/u/agoldfa7/research/plots/" + filename + ".png", bbox_inches="tight")
	plt.clf()

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
	(ab,c,t) = collect_freq()

	# print(ab)
	# print(c)
	# print(t)
	graph_scatter(ab,"Reuse distance distribution on mm size 16x16 (AB)","Reuse distance", "Frequency","ab")
	graph_scatter(c,"Reuse distance distribution on mm size 16x16 (C)","Reuse distance", "Frequency","c")
	graph_scatter(t,"Reuse distance distribution on mm size 16x16 (Temp)","Reuse distance", "Frequency","temp")

if __name__ == "__main__":
	main()
