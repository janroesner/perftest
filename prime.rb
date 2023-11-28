def sieve_of_eratosthenes(max)
  primes = Array.new(max + 1, true)
  primes[0] = primes[1] = false

  (2..Math.sqrt(max)).each do |i|
    if primes[i]
      (i*i).step(max, i) { |j| primes[j] = false }
    end
  end

  primes.each_index.select { |i| primes[i] }
end

def main
  if ARGV.length != 2
    puts "Usage: ruby prime_calculator.rb <max_prime> <num_iterations>"
    exit
  end

  max_prime = ARGV[0].to_i
  num_iterations = ARGV[1].to_i

  num_iterations.times do
    primes = sieve_of_eratosthenes(max_prime)
    puts "Found #{primes.length} primes up to #{max_prime}"
  end
end

main

