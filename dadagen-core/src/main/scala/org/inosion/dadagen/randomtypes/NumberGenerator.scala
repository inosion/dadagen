package org.inosion.dadagen.randomtypes

import java.util.Random

import org.inosion.dadagen.Context

/**
 * @author rbuckland
 */
case class IntegerGenerator(name:String,min:Int,max:Int)(implicit rand: Random) extends DataGenerator[Int] {

  override def description: String = "Create a random integer (integer)"

  override def internalGenerate(context: Context)(implicit rand: Random): Int = RandomUtil.randomIntRange(min,max)
}

case class DoubleGenerator(name:String,min:Double,max:Double,precision:Int=2)(implicit rand: Random) extends DataGenerator[Double] {

  override def description: String = "Create a random integer (integer)"

  override def internalGenerate(context: Context)(implicit rand: Random): Double = RandomUtil.randomDoubleRange(min,max,precision)

}

/**
 * Support object for creating the right type of "number" generator
 */
object NumberGenerator {
  def createNumberGen(name:String, min:String,max:String,precision:Int):DataGenerator[_] = {
    precision match {
      case 0 => IntegerGenerator(name,min.toInt,max.toInt)
      case _ => DoubleGenerator(name,min.toDouble,max.toDouble,precision)
    }
  }
}
