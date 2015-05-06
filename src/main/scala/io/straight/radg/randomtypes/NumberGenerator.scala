package io.straight.radg.randomtypes

import io.straight.radg.{RandomUtil, Context}

/**
 * @author rbuckland
 */
case class IntegerGenerator(name:String,min:Int,max:Int) extends DataGenerator[Int] {

  override def description: String = "Create a random integer (integer)"

  override def internalGenerate(context: Context): Int = RandomUtil.randomIntRange(min,max)
}

case class DoubleGenerator(name:String,min:Double,max:Double,precision:Int=2) extends DataGenerator[Double] {

  override def description: String = "Create a random integer (integer)"

  override def internalGenerate(context: Context): Double = RandomUtil.randomDoubleRange(min,max,precision)

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
