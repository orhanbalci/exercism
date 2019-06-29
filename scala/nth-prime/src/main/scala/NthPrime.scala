import scala.collection.immutable.Stream;

object NthPrime {
  def prime(n: Int): Option[Int] = {
    var primes = List[Int]()
    for (i <- 2 to Int.MaxValue) {
      if (primes.length == n)
        return primes.headOption
      if (!primes.exists(x => i % x == 0) || i == 2)
        primes = i :: primes
    }
    return None
  }
}
