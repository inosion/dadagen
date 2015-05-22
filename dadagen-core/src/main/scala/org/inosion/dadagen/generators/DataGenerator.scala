package org.inosion.dadagen.generators

import org.inosion.dadagen.Context

import java.util.Random

/**
 * @author rbuckland
 */
trait DataGenerator[T] {

  def name: String
  def description: String
  def generate(context:Context,dependantList:List[String])(implicit rand: Random):T = {
    val result = internalGenerate(context)
    if (dependantList.contains(name)) {
      context.dataFieldState.put(name, result)
    }
    result
  }

  def internalGenerate(context:Context)(implicit rand: Random):T

  /*
   * Pre execution derived list of dependencies that this field is waiting on
   */
  def dependencies: List[String] = List.empty

}

object DataGenerator {
  implicit val rand: java.util.Random = org.inosion.dadagen.rand
}