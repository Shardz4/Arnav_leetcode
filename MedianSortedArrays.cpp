#include <vector>
#include <algorithm>
#include <climits>

class Solution {
    public:
    double findMedianSortedArrays(std::cector<int> & nums 1, std::vector<int> & nums2){
        int m = nums1.size();
        int n = nums2.size();
        if (m > n) {
            return findMedianSortedArrays(nums2, nums1); // Ensure nums1 is the smaller array
        }

        int low = 0, high = m;
        while (low <=high){
            int i = (low + high) / 2;
            int j = (m + n + 1) / 2 - i;

            int left1 = (i == 0) ? INT_MIN : nums1[i - 1];
            int left2 = (j == 0) ? INT_MIN : nums2[j - 1];
            int right1 = (i == m) ? INT_MAX : nums1[i];
            int right2 = (j == n) ? INT_MAX : nums2[j];

            if (left1 <= right2 && left2 <= right1) {
                if ((m + n) % 2 == 1) {
                    return std::max(left1, left2);
                }
                else{
                return (std::max(left1, left2) + std::min(right1, right2)) / 2.0;
                }
            }
            else if (left1 > right2) {
                high = i - 1; 
            } 
            else {
                low = i + 1; 
            }
        }
        return 0.0;
    }
};