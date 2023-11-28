function sieveOfEratosthenes(maxPrime) {
    const primes = new Array(maxPrime + 1).fill(true);
    primes[0] = primes[1] = false;

    for (let i = 2; i <= Math.sqrt(maxPrime); i++) {
        if (primes[i]) {
            for (let j = i * i; j <= maxPrime; j += i) {
                primes[j] = false;
            }
        }
    }

    return primes.map((prime, index) => prime ? index : null).filter((val) => val !== null);
}

function main() {
    const [,, maxPrime, numIterations] = process.argv;

    if (!maxPrime || !numIterations) {
        console.error("Usage: node prime_calculator.js <max_prime> <num_iterations>");
        process.exit(1);
    }

    for (let i = 0; i < numIterations; i++) {
        const primes = sieveOfEratosthenes(Number(maxPrime));
        console.log(`Found ${primes.length} primes up to ${maxPrime}`);
    }
}

main();

