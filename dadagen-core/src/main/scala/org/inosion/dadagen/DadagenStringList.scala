package org.inosion.dadagen

import org.inosion.dadagen.Context

import scala.collection.JavaConverters._


import org.inosion.dadagen.generators.Generator

import java.util.Random


/**
 * Create a List of List of strings (used for direct output to CSV or some such)
 * The header, field names are found by calling fieldNames
 */
case class DadagenStringList(
    generators:List[Generator[_]] = List.empty
)(implicit rand: Random) extends Dadagenerator[List[String]] {

  /**
   * Map each generator (in deps order) to a list of Strings (values)
   * And then re-sort the list according to the original order
   */
  override def fgen(ctx: Context): List[String] = {
    val row = generatorsOrder map (g => g.generate(ctx, dependentOn).toString)
    // re-sort it
    dependentOn.isEmpty match {
      case true => row
      case false => row.zip(dependencySortIndex).sortBy(_._2).map(_._1)
    }
  }
}



