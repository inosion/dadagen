package org.inosion.datdom.randomtypes

import org.inosion.datdom.Context

/**
 * @author rbuckland
 */
trait DataGenerator[T] {

  def name: String
  def description: String
  def generate(context:Context,dependantList:List[String]):T = {
    val result = internalGenerate(context)
    if (dependantList.contains(name)) {
      context.dataFieldState.put(name, result)
    }
    result
  }

  def internalGenerate(context:Context):T

  /*
   * Pre execution derived list of dependencies that this field is waiting on
   */
  def dependencies: List[String] = List.empty

}
