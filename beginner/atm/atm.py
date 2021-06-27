from sys import exit

# Entity
class Account:
    
    def __init__(
        self,
        balance: float):
        Account.raise_if_balance_is_negative(balance)
        self._balance = balance

    def get_balance(self) -> float:
        return self._balance
    
    def deposit(self, amount: int):
        try:
            if amount <= 0:
                raise ValueError("ERROR: Deposit Amount cannot be less than or equal to 0")

            self._balance += amount
        except TypeError as e:
            print(e)
    
    def withdraw(self, amount: int):
        try:
            if amount > self._balance:
                raise ValueError("ERROR: Withdraw Amount cannot be greater than the balance")
            if self._balance <= 0:
                raise ValueError("ERROR: Withdraw Amount cannot be less than or equal to 0")

            self._balance -= amount
        except ValueError as e:
             print(e)
    
    @staticmethod
    def raise_if_balance_is_negative(value: float):
        if value < 0:
            raise ValueError("ERROR: Negative value not allowed!")

# Interactor
class ABCBank(Account):

    bank_charges: float = 0.5

    def __init__(
        self,
        balance: float = 0.0):

        ABCBank.raise_if_greater_than_2000(balance=balance)
        super().__init__(balance)

    def withdraw(self, amount: int):

        ABCBank.raise_if_greater_than_2000(amount=amount)

        if abs(amount)%5 != 0:
            raise ValueError("ERROR: Transaction amount must be a multiple of 5 only")
        if (ABCBank.bank_charges + abs(amount)) > self._balance:
            raise ValueError("ERROR: Insufficient account balance")

        super().withdraw(amount)
        super().withdraw(ABCBank.bank_charges)
    
    @classmethod
    def set_bank_charges(cls, new_charges):
        cls.bank_charges = new_charges
    
    @staticmethod
    def raise_if_greater_than_2000(**kwargs):
        for (key, val) in kwargs.items():
            if val > 2000:
                raise ValueError(f"ERROR: {key} cannot be greater than 2000")


# Boundary
class ATM:

    @staticmethod
    def input_balance() -> float:
        return float(input("Enter Current Balance: "))
    
    @staticmethod
    def input_withdraw_amount() -> int:
        return int(input("Enter Withdraw Amount: "))
    
    @staticmethod
    def input_if_continue():
        return input("Do you want to Continue? (Y/N): ")

# I/O
def main():
    try:
        balance: float = ATM.input_balance()
        pooja_account = ABCBank(balance)
    except Exception as e:
        print(e)
        exit(0)
    
    while True:
        try:
            withdraw_amount: int = ATM.input_withdraw_amount()

            pooja_account.withdraw(withdraw_amount)
            print(f"Pooja's Balance after transaction: {pooja_account.get_balance()}")
        except Exception as e:
            print(e)
        finally:
            if_continue = ATM.input_if_continue()
            if if_continue.lower() == "n":
                print("Thank you for transacting with us!")
                exit(0)
            continue

if __name__ == "__main__":
    main()
