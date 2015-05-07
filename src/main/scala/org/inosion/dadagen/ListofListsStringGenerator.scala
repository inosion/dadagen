package org.inosion.dadagen

import org.inosion.dadagen.randomtypes.DataGenerator


abstract class StringsGeneratorSupport[A](
   generators: List[DataGenerator[_]])
extends RandomDataGenerator[Iterator[A]] {

  def fgen(ctx:Context):A

  def generateAll(rows: Int):List[A] = generate().take(rows).toList

  /**
   * All the column names, or if a map the Key Names
   */
  val fieldNames = generators.map(_.name)

  override def generate(): Iterator[A] = new AbstractIterator[A] {
    private[this] var i = 0
    override def hasNext: Boolean = true
    override def next(): A = {
      i = i+1
      val ctx:Context = new Context(i) // holds the values for this row
      fgen(ctx)
    }
  }


}
/**
 * Create a List of List of strings (used for direct output to CSV or some such)
 * The header, field names are found by calling fieldNames
 */
case class ListOfStringsGenerator(
                                       generators:List[DataGenerator[_]] = List.empty
                                       ) extends StringsGeneratorSupport[List[String]](generators) {

  override def fgen(ctx: Context): List[String] = {
    val row = generatorsOrder map (g => g.generate(ctx, dependentOn).toString)
    // re-sort it
    dependentOn.isEmpty match {
      case true => row
      case false => row.zip(dependencySortIndex).sortBy(_._2).map(_._1)
    }
  }
}

/**
 * Create a Map, one map would represent one "row" or one "session" set of data
 * Key names can be found before hand by calling "fieldnames"
 *
 * @param generators
 */
case class MapOfStringsGenerator(
    generators:List[DataGenerator[_]] = List.empty
) extends StringsGeneratorSupport[Map[String,String]](generators) {

  override def fgen(ctx:Context):Map[String,String] =
     generatorsOrder map (g => g.name -> g.generate(ctx, dependentOn).toString) toMap

}



