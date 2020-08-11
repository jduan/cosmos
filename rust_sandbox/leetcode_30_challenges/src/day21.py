# Leftmost Column with at Least a One
#
# (This problem is an interactive problem.)
#
# A binary matrix means that all elements are 0 or 1. For each individual row of the matrix,
# this row is sorted in non-decreasing order.
#
# Given a row-sorted binary matrix binaryMatrix, return leftmost column index(0-indexed) with at
# least a 1 in it. If such index doesn't exist, return -1.
#
# You can't access the Binary Matrix directly.  You may only access the matrix using a
# BinaryMatrix interface:
#
#     BinaryMatrix.get(x, y) returns the element of the matrix at index (x, y) (0-indexed).
#     BinaryMatrix.dimensions() returns a list of 2 elements [n, m], which means the matrix is n
#     * m.
#
# Submissions making more than 1000 calls to BinaryMatrix.get will be judged Wrong Answer.  Also,
# any solutions that attempt to circumvent the judge will result in disqualification.
#
# For custom testing purposes you're given the binary matrix mat as input in the following four
# examples. You will not have access the binary matrix directly.
#
#
#
# Example 1:
#
# Input: mat = [[0,0],[1,1]]
# Output: 0
#
# Example 2:
#
# Input: mat = [[0,0],[0,1]]
# Output: 1
#
# Example 3:
#
# Input: mat = [[0,0],[0,0]]
# Output: -1
#
# Example 4:
#
# Input: mat = [[0,0,0,1],[0,0,1,1],[0,1,1,1]]
# Output: 1
#
#
#
# Constraints:
#
#     1 <= mat.length, mat[i].length <= 100
#     mat[i][j] is either 0 or 1.
#     mat[i] is sorted in a non-decreasing way.


class BinaryMatrix(object):
    def __init__(self, matrix):
        self.matrix = matrix

    def get(self, x, y):
        """
        :type x : int, y : int
        :rtype int
        """
        # print("Get matrix[%s][%s]" % (x, y))
        return self.matrix[x][y]

    def dimensions(self):
        """
        :rtype list[]
        """
        return [len(self.matrix), len(self.matrix[0])]


class Solution(object):
    def leftMostColumnWithOne(self, binaryMatrix):
        """
        :type binaryMatrix: BinaryMatrix
        :rtype: int

        The idea is that we start from the top right corner. If it's a 1, we move to the left. If
        it's a 0, we move down. The invariant is that at any time, we are already at the column
        that has a 1 and we are trying to improve ourselves by finding a column that also has a 1
        and the column is further to the left.
        """
        rows, cols = binaryMatrix.dimensions()
        row = 0
        col = cols - 1
        while row < rows and col >= 0:
            value = binaryMatrix.get(row, col)
            if value == 1: # move to the left
                col -= 1
            elif value == 0: # move down
                row += 1
            else:
                raise "Invalid value: %s" % value

        if col == cols - 1:
            return -1
        else:
            return col + 1


def main():
    binary_matrix = BinaryMatrix([[0,0,0,1],[0,0,1,1],[0,1,1,1]])
    print(1 == Solution().leftMostColumnWithOne(binary_matrix))

    binary_matrix = BinaryMatrix([[0,0],[1,1]])
    print(0 == Solution().leftMostColumnWithOne(binary_matrix))

    binary_matrix = BinaryMatrix([[0,0],[0,1]])
    print(1 == Solution().leftMostColumnWithOne(binary_matrix))

    binary_matrix = BinaryMatrix([[0,0],[0,0]])
    print(-1 == Solution().leftMostColumnWithOne(binary_matrix))


if __name__ == '__main__':
    main()
