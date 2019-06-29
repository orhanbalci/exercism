object DifferenceOfSquares {

  def sumOfSquares(n: Int): Int = (1 to n).map(x => x * x).foldLeft(0)(_ + _)

  def squareOfSum(n: Int): Int = math.pow((n * (n + 1) / 2), 2).toInt

  def differenceOfSquares(n: Int): Int = squareOfSum(n) - sumOfSquares(n)
}
