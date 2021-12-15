import unittest
from email_transaction_to_budget import classify, BudgetCategory


class Test(unittest.TestCase):
    def test_toast_prefix_is_food_category(self):
        self.assertEqual(classify("TST* adsasjd", "", ""), BudgetCategory.food)

    def test_food_vendor_list(self):
        self.assertEqual(classify("hy-vee", "", ""), BudgetCategory.food)


if __name__ == "__main__":
    unittest.main()
