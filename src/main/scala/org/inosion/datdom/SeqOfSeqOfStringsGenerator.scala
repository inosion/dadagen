package org.inosion.datdom

import org.inosion.datdom.randomtypes.DataGenerator

/**
 * Create a Seq of Seq of strings (used for direct output to CSV or some such)
 * The Tuple holds the Column Names -> Data
 */
case class SeqOfSeqOfStringsGenerator(

  generators:List[DataGenerator[_]] = List.empty

) extends RandomDataGenerator[Iterator[List[String]]] {

  val fieldNames = generators.map(_.name)

  /**
   * Helper method to generate a fixed number of rows
   * @param rows
   * @return
   */
  def generateAll(rows: Int): (List[String], List[List[String]]) = ( fieldNames , generate().take(rows).toList)

  /**
   * An Iterator to provide a continuous Stream of Random Rows
   * @return
   */
  override def generate(): Iterator[List[String]] = new AbstractIterator[List[String]] {
    private[this] var i = 0
    override def hasNext: Boolean = true
    override def next(): List[String] = {
      i = i+1
      val ctx:Context = new Context(i) // holds the values for this row
      val row = for {
            generator <- generatorsOrder // this is the correct generation order, but it mucks up the order
            valGen = generator.generate(ctx,dependentOn)
          } yield {
            valGen.toString
          }

      dependentOn.isEmpty match {
        case true => row
        case false => row.zip(dependencySortIndex).sortBy(_._2).map(_._1)
      }

    }
  }

}
