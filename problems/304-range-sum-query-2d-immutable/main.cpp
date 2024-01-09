#include <vector>
using namespace std;

class NumMatrix {
private:
  vector<vector<int>> preSum;

public:
  NumMatrix(vector<vector<int>> &matrix) {
    int m = matrix.size();
    int n = matrix[0].size();
    preSum = vector<vector<int>>(m + 1, vector<int>(n + 1, 0));
    for (int i = 0; i < m; i++) {
      for (int j = 0; j < n; j++) {
        preSum[i + 1][j + 1] =
            preSum[i][j + 1] + preSum[i + 1][j] - preSum[i][j] + matrix[i][j];
      }
    }
  }

  int sumRegion(int row1, int col1, int row2, int col2) {
    return preSum[row2 + 1][col2 + 1] + preSum[row1][col1] -
           preSum[row1][col2 + 1] - preSum[row2 + 1][col1];
  }
};
