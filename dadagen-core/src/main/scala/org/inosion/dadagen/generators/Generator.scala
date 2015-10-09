package org.inosion.dadagen.generators

import java.util.Random

import org.inosion.dadagen.Context


/**
 * @author rbuckland
 *
 */
trait Generator[T] {

  def name: String
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

object Generator {
  implicit val rand: java.util.Random = org.inosion.dadagen.rand
}