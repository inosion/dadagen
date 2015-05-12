package org.inosion.dadagen.randomtypes

import java.util.Random


/**
 * @author rbuckland
 */
object RandomUtil {

  /**
   * Pull a random value from a string separated list
   * @param commaSeparatedList
   * @return
   */
  def randomFromStringSeparatedList(commaSeparatedList: String)(implicit rand:Random): String = {
    val list: Array[String] = commaSeparatedList.split(",")
    return list(rand.nextInt(list.length)).trim
  }

  /*
   * Random integer from 1 to upper number inclusive
   */
  def randomIntUpto(int:Integer)(implicit rand:Random):Int = rand.nextInt(int) + 1


  /**
   * An inclusive Integer range
   * @param min
   * @param max
   * @return
   */
  def randomIntRange(min:Int,max:Int)(implicit rand:Random) = rand.nextInt(max - min) + 1 + min

  /**
   * Double generator
   * @param min
   * @param max
   * @param decimalPlaces
   * @return
   */
  def randomDoubleRange(min:Double,max:Double,decimalPlaces:Int)(implicit rand:Random) =
    BigDecimal(min + (rand.nextDouble() * (max - min))).setScale(decimalPlaces, BigDecimal.RoundingMode.HALF_UP).toDouble

  /**
   * Pull a random value from the list
   * @param alist
   * @tparam E
   * @return
   */
  def randomFromList[E](alist: Seq[E])(implicit rand:Random): E = alist(rand.nextInt(alist.size))

  def randomWeightedIndex[T: Numeric](weights:List[T]): Int = {
    import scala.math.Numeric.Implicits._
    val sum = weights.sum.toFloat()
    val rnd = rand.nextFloat()
    println(s"${sum} ${rnd}")
    weights.zipWithIndex.find(x => rnd < (x._1.toFloat() / sum)) match {
      case None => weights.size-1
      case Some((_,idx)) => idx
    }
  }

  /**
   * The index is a list of "integers", that are the positions in the main list
   *
   * @param alist
   * @param index
   * @tparam E
   * @return
   */
  def randomFromListByIndex[E](alist: Seq[E],index:Seq[Int])(implicit rand:Random):E = alist(index(rand.nextInt(index.size)))

}
