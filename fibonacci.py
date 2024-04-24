from timeit import default_timer as timer

def fibonacci(n):
    if n < 2:
        return n
    else:
        return fibonacci(n - 1) + fibonacci(n - 2)

def main():
    n = int(input("Enter n:"))
    if n > 93:
        print("Please enter a number less than 94.")
        main()
    else:
        start_time = timer()
        for i in range(1):
            _ = fibonacci(n)
        average_time = timer() - start_time / 20.0
            
        print(f"The {n}-th Fibonacci numbers is: {fibonacci(n)}")
        print(f"It took on average {average_time}s to calculate this number with 20 benchmarks.")

main()