import sys

def sieve_of_eratosthenes(max_prime):
    primes = [True] * (max_prime + 1)
    primes[0] = primes[1] = False
    for i in range(2, int(max_prime**0.5) + 1):
        if primes[i]:
            for j in range(i*i, max_prime + 1, i):
                primes[j] = False
    return [i for i, prime in enumerate(primes) if prime]

def main():
    if len(sys.argv) != 3:
        print("Usage: python prime_calculator.py <max_prime> <num_iterations>")
        sys.exit(1)

    max_prime = int(sys.argv[1])
    num_iterations = int(sys.argv[2])

    for _ in range(num_iterations):
        primes = sieve_of_eratosthenes(max_prime)
        print(f"Found {len(primes)} primes up to {max_prime}")

if __name__ == "__main__":
    main()

