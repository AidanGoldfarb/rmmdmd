import numpy as np
import sys
import math
trace = []
temp = 1
n_start = 0

def mat_mul(a,b):
    global trace
    n = len(a)
    c = np.zeros((n,n), dtype=object)
    if n == 1:
        trace.append(str(a[0][0]))
        trace.append(str(b[0][0]))
        c[0][0] = tempname()
        trace.append(c[0][0])
    else:
        #A11B11 + A12B21
        c[0:n//2, 0:n//2] = matrix_add(mat_mul(a[0:n//2, 0:n//2], b[0:n//2, 0:n//2]), mat_mul(a[0:n//2, n//2:n], b[n//2:n, 0:n//2]))
        #A11B12 + A12B22
        c[0:n//2, n//2:n] = matrix_add(mat_mul(a[0:n//2, 0:n//2], b[0:n//2, n//2:n]), mat_mul(a[0:n//2, n//2:n], b[n//2:n, n//2:n]))
        #A21B11 + A22B21
        c[n//2:n, 0:n//2] = matrix_add(mat_mul(a[n//2:n, 0:n//2], b[0:n//2, 0:n//2]), mat_mul(a[n//2:n, n//2:n], b[n//2:n, 0:n//2]))
        #A21B12 + A22B22
        c[n//2:n, n//2:n] = matrix_add(mat_mul(a[n//2:n, 0:n//2], b[0:n//2, n//2:n]), mat_mul(a[n//2:n, n//2:n], b[n//2:n, n//2:n]))
    return c


def tempname():
    global temp
    tmp = 't' + str(temp)
    temp = temp + 1
    return tmp

def matrix_add(a,b):
    global temp
    global trace
    n = len(a)
    result = np.zeros((n,n), dtype=object)
    for i in range(n):
        for j in range(n):
            trace.append(a[i,j])
            trace.append(b[i,j])
            result[i,j] = tempname()
            trace.append(result[i,j])
    # if n>1:
    #     print(result)
    return result



def process_trace():
    stack = []
    rds = {}
    dist = {}
    with open("output.txt", "w") as f:
        #A_B -> A_B
        #f.write("##### reuse distribution #####\n")
        for i in range(len(trace)):
            if trace[i] not in stack:
                stack.append(trace[i])
                rds[trace[i]] = []
            else:
                ind = stack.index(trace[i])
                rd = len(stack)-ind
                rds[trace[i]].append(rd)
                stack.pop(ind)
                stack.append(trace[i])
        for (k,v) in rds.items():
            for val in v:
                if val not in dist:
                    dist[val] = 1
                else:
                    dist[val] = dist[val]+1
        for (k,v) in sorted(dist.items()):
            f.write(str(k) + ": " + str(v) + "\n")
        # for lvl in range(int(math.log(n_start, 2))):
        #     f.write('###L' + str(2**lvl) + '###\n')
        #     f.write('A:\n')
        #     for i in range(n_start):
        #         f.write('[')
        #         for j in range(1,n_start+1):
        #             datum = str(n_start * i + j)
        #             val = min(rds[datum])
        #             f.write(str(val) + ', ')
        #             rds[datum] = [x for x in rds[datum] if x != val]
        #         f.write(']\n')
        #     f.write('B:\n')
        #     for i in range(n_start):
        #         f.write('[')
        #         for j in range(1,n_start+1):
        #             datum = str(n_start * i + j + n_start*n_start)
        #             val = min(rds[datum])
        #             f.write(str(val) + ', ')
        #             rds[datum] = [x for x in rds[datum] if x != val]
        #         f.write(']\n')

        # #A_B -> A_B (SHIFTED)
        # f.write("##### A_B -> A_B SHIFTED #####\n")
        # for i in range(len(trace)):
        #     if trace[i] not in stack and trace[i][0] != 't':
        #         stack.append(trace[i])
        #         rds[trace[i]] = []
        #     elif trace[i][0] != 't':
        #         ind = stack.index(trace[i])
        #         rd = len(stack)-ind
        #         rds[trace[i]].append(rd)
        #         stack.pop(ind)
        #         stack.append(trace[i])
        #     else:
        #         continue
        # for lvl in range(int(math.log(n_start, 2))):
        #     f.write('###L' + str(2**lvl) + '###\n')
        #     f.write('A:\n')
        #     for i in range(n_start):
        #         f.write('[')
        #         for j in range(1,n_start+1):
        #             datum = str(n_start * i + j)
        #             val = min(rds[datum])
        #             f.write(str(val - 4*(2**lvl)**2) + ', ')
        #             rds[datum] = [x for x in rds[datum] if x != val]
        #         f.write(']\n')
        #     f.write('B:\n')
        #     for i in range(n_start):
        #         f.write('[')
        #         for j in range(1,n_start+1):
        #             datum = str(n_start * i + j + n_start*n_start)
        #             val = min(rds[datum])
        #             f.write(str(val - 6*(4**lvl)) + ', ')
        #             rds[datum] = [x for x in rds[datum] if x != val]
        #         f.write(']\n')


        # #A_B -> TMP
        # stack = []
        # f.write("##### A_B -> TMP #####\n")
        # for i in range(len(trace)):
        #     if trace[i] not in stack:
        #         stack.append(trace[i])
        #         rds[trace[i]] = []
        #     else:
        #         rd = len([x for x in stack[stack.index(trace[i]):] if x[0] == 't'])
        #         rds[trace[i]].append(rd)
        #         stack.pop(stack.index(trace[i]))
        #         stack.append(trace[i])
            
        # for lvl in range(int(math.log(n_start, 2))):
        #     f.write('###L' + str(2**lvl) + '###\n')
        #     f.write('A:\n')
        #     for i in range(n_start):
        #         f.write('[')
        #         for j in range(1,n_start+1):
        #             datum = str(n_start * i + j)
        #             val = min(rds[datum])
        #             f.write(str(val) + ', ')
        #             rds[datum] = [x for x in rds[datum] if x != val]
        #         f.write(']\n')
        #     f.write('B:\n')
        #     for i in range(n_start):
        #         f.write('[')
        #         for j in range(1,n_start+1):
        #             datum = str(n_start * i + j + n_start*n_start)
        #             val = min(rds[datum])
        #             if val == 52:
        #                 print(datum)
        #             f.write(str(val) + ', ')
        #             rds[datum] = [x for x in rds[datum] if x != val]
        #         f.write(']\n')


        # #A_B -> TOTAL
        # f.write("\n##### A_B -> TOTAL #####\n")
        # stack = []
        # for i in range(len(trace)):
        #     if trace[i] not in stack:
        #         stack.append(trace[i])
        #         rds[trace[i]] = []
        #     else:
        #         ind = stack.index(trace[i])
        #         rd = len(stack)-ind
        #         rds[trace[i]].append(rd)
        #         stack.pop(ind)
        #         stack.append(trace[i])
        # for lvl in range(int(math.log(n_start, 2))):
        #     f.write('###L' + str(2**lvl) + '###\n')
        #     f.write('A:\n')
        #     for i in range(n_start):
        #         f.write('[')
        #         for j in range(1,n_start+1):
        #             datum = str(n_start * i + j)
        #             val = min(rds[datum])
        #             f.write(str(val) + ', ')
        #             rds[datum] = [x for x in rds[datum] if x != val]
        #         f.write(']\n')
        #     f.write('B:\n')
        #     for i in range(n_start):
        #         f.write('[')
        #         for j in range(1,n_start+1):
        #             datum = str(n_start * i + j + n_start*n_start)
        #             val = min(rds[datum])
        #             f.write(str(val) + ', ')
        #             rds[datum] = [x for x in rds[datum] if x != val]
        #         f.write(']\n')
        


        # #TMP -> TOTAL
        # f.write("\n##### TMP -> TOTAL #####\n")
        # tmp_dist = {}
        # sorted_dists = []
        # sorted_counts = []
        # tmps = []
        # for datum in rds:
        #     if datum[0] =='t':
        #         if len(rds[datum])>0:
        #             tmps.append(datum)
        # for datum in tmps:
        #     if rds[datum][0] not in tmp_dist:
        #         tmp_dist[rds[datum][0]] = 1
        #     else:
        #         tmp_dist[rds[datum][0]] = tmp_dist[rds[datum][0]] + 1
        # for dist in tmp_dist:
        #     sorted_dists.append(dist)
        # sorted_dists.sort()
        # for dist in sorted_dists:
        #     sorted_counts.append(tmp_dist[dist])
        # for i in range(len(sorted_dists)):
        #     f.write(str(sorted_dists[i]) + ': ' + str(sorted_counts[i]) + '\n')


        # f.write("##### TMPS AS MATRICES #####\n")
        # tmps_by_level = [[] for i in range(int(math.log(n_start, 2)))]
        # for val in tmps:
        #     num = int(val[1:])
        #     if num >= (n_start*n_start)*(2*n_start - 1)//4:
        #         break
        #     n_temp = n_start
        #     level = int(math.log(n_start, 2))
        #     assigned = False
        #     while not assigned:
        #         if num % ((n_temp*n_temp)*(2*n_temp - 1)//4) >= ((n_temp*n_temp)*(2*n_temp - 1)//4 - (n_temp//2)*(n_temp//2) + 1) or num % ((n_temp*n_temp)*(2*n_temp - 1)//4) == 0:
        #             tmps_by_level[level].append(val)
        #             assigned = True
        #         else:
        #             if n_temp == 2:
        #                 tmps_by_level[0].append(val)
        #                 assigned = True
        #             else:
        #                 n_temp = n_temp//2
        #                 level = level-1
        #                 num = num % ((n_temp*n_temp)*(2*n_temp - 1)//4)
        
        # for k in range(1, int(math.log(n_start, 2))):
        #     level = 2**k
        #     quadrant_dim = level//2
        #     #f.write('L' + str(level) + ':\n')
        #     #for l in range(len(tmps_by_level[k])//(level*level//4)):
        #     while len(tmps_by_level[k])>0:
        #         #f.write('\n')
        #         matrix = np.zeros((level, level), dtype=object)
        #         for m in range(2):
        #             for n in range(2):
        #                 quadrant = np.zeros((quadrant_dim, quadrant_dim), dtype=object)
        #                 for i in range(quadrant_dim):
        #                     #f.write('[')
        #                     for j in range(quadrant_dim):
        #                         item = tmps_by_level[k].pop(0)
        #                         quadrant[i][j] = item
        #                 matrix[m*quadrant_dim:m*quadrant_dim+quadrant_dim, n*quadrant_dim:n*quadrant_dim+quadrant_dim] = quadrant
        #         f.write('\n')
        #         for row in matrix:
        #             f.write("\n[")
        #             for item in row:
        #                 f.write(str(rds[item][0]) + ', ')
        #                 if rds[item][0] == 635:
        #                     print(item)
        #             f.write("]")

if __name__ == "__main__":
    n_start = int(sys.argv[1])
    a = np.arange(n_start*n_start).reshape(n_start,n_start)
    b = np.arange(n_start*n_start).reshape(n_start,n_start)
    for i in range(n_start):
        for j in range(n_start):
            a[i,j] = a[i,j] + 1
            b[i,j] = b[i,j] + (n_start*n_start) + 1
    mat_mul(a,b)
    process_trace()

    
    
	
	
	
