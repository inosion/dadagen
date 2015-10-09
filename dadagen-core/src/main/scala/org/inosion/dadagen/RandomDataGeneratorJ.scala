package org.inosion.dadagen

import scala.collection.JavaConverters._

/**
 * Java Implementation of the RandomDataGenerator
 *
 * Appending 'J' Little bit code smelly (TODO : resolve a better Java API design)
 *
 * @tparam A
 */
abstract class RandomDataGeneratorJ[A] extends Dadagenerator[A] {

  def generateAllJ(rows: java.lang.Integer) = super.generateAll(rows).asJava

  def generateJ(): java.util.Iterator[A] = super.generate().asJava

}


