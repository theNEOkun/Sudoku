base  = 3
side  = base*base

# pattern for a baseline valid solution
def pattern(r,c): return (base*(r%base)+r//base+c)%side

# randomize rows, columns and numbers (of valid base pattern)
from random import sample
def shuffle(s): 
    print("s", s)
    return sample(s,len(s)) 

[6, 2, 5, 8, 4, 3, 7, 9, 1]
[7, 9, 1, 2, 6, 5, 4, 8, 3]
[4, 8, 3, 9, 7, 1, 6, 2, 5]
[8, 1, 4, 5, 9, 7, 2, 3, 6]
[2, 3, 6, 1, 8, 4, 9, 5, 7]
[9, 5, 7, 3, 2, 6, 8, 1, 4]
[5, 6, 9, 4, 3, 2, 1, 7, 8]
[3, 4, 2, 7, 1, 8, 5, 6, 9]
[1, 7, 8, 6, 5, 9, 3, 4, 2]

def create_list(rBase):
    retlist = []
    for r in shuffle(rBase):
        for g in shuffle(rBase):
            print(r, g)
            retlist.append(g*base + r)

    return retlist

def numser(rows, cols):
    retlist = []
    nums = shuffle(range(1,base*base+1))
    print(nums)
    for r in rows:
        for c in cols:
            retlist.append(nums[pattern(r, c)])

    retlist


def main():
    rBase = range(base) 
    rows  = create_list(rBase) 
    cols  = create_list(rBase)

    # produce board using randomized baseline pattern
    board = numser(rows, cols)
    print(rows)
    print(cols)

    #for line in board: print(line)


if __name__ == '__main__':
    main()
