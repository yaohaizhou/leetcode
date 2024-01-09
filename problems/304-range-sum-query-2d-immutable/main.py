from typing import List

class NumMatrix:

    def __init__(self, matrix: List[List[int]]):
        m = len(matrix)
        n = len(matrix[0])
        self.preSum = [[0 for _ in range(n+1)] for _ in range(m+1)]
        for i in range(m):
            for j in range(n):
                self.preSum[i+1][j+1] = (self.preSum[i][j+1] +
                                         self.preSum[i+1][j] -
                                         self.preSum[i][j] +
                                         matrix[i][j])

    def sumRegion(self, row1: int, col1: int, row2: int, col2: int) -> int:
        return (self.preSum[row2+1][col2+1] +
                self.preSum[row1][col1] -
                self.preSum[row1][col2+1] -
                self.preSum[row2+1][col1])
