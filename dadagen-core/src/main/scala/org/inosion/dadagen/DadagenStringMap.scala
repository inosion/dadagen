package org.inosion.dadagen

import java.util.Random
import scala.language.postfixOps

import org.inosion.dadagen.generators.Generator

/**
 * Create a Map, one map would represent one "row" or one "session" set of data
 *
 * Key names can be found before hand by calling "fieldnames"
 *
 * @param generators
 */
case class DadagenStringMap(
                            generators:List[Generator[_]] = List.empty
                            )(implicit rand: Random) extends Dadagenerator[Map[String,String]] {

  /**
   * For each generator, in dependency order, create a map of "key name" => value.toString
   * Order of generated data in the Map does not matter
   */
  override def fgen(ctx:Context):Map[String,String] =
    generatorsOrder map (g => g.name -> g.generate(ctx, dependentOn).toString) toMap

}
