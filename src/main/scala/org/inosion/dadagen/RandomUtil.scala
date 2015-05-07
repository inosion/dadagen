package org.inosion.dadagen

import java.security.SecureRandom
import org.inosion.dadagen.randomtypes.{CountryCode}

/**
 * @author rbuckland
 */
object RandomUtil {

  private var rand: SecureRandom = new SecureRandom

  /**
   * Pull a random value from a string separated list
   * @param commaSeparatedList
   * @return
   */
  def randomFromStringSeparatedList(commaSeparatedList: String): String = {
    val list: Array[String] = commaSeparatedList.split(",")
    return list(rand.nextInt(list.length)).trim
  }

  /*
   * Random integer from 1 to upper number inclusive
   */
  def randomIntUpto(int:Integer):Int = rand.nextInt(int) + 1


  /**
   * An inclusive Integer range
   * @param min
   * @param max
   * @return
   */
  def randomIntRange(min:Int,max:Int) = rand.nextInt(max - min) + 1 + min

  /**
   * Double generator
   * @param min
   * @param max
   * @param decimalPlaces
   * @return
   */
  def randomDoubleRange(min:Double,max:Double,decimalPlaces:Int) =
    BigDecimal(min + (rand.nextDouble() * (max - min))).setScale(decimalPlaces, BigDecimal.RoundingMode.HALF_UP).toDouble

  /**
   * Pull a random value from the list
   * @param alist
   * @tparam E
   * @return
   */
  def randomFromList[E](alist: Seq[E]): E = alist(rand.nextInt(alist.size))

  /**
   * The index is a list of "integers", that are the positions in the main list
   *
   * @param alist
   * @param index
   * @tparam E
   * @return
   */
  def randomFromListByIndex[E](alist: Seq[E],index:Seq[Int]):E = alist(index(rand.nextInt(index.size)))

}
