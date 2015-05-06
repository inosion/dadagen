package io.straight.radg

import io.straight.radg.randomtypes.DataGenerator

/**
 * Create an Seq of Seq of strings (used for direct output to CSV or some such)
 * The Tuple holds the Column Names -> Data
 */
case class SeqOfSeqOfStringsGenerator(
  rows:Int,
  // the rowConfig is an ordering
  generators:Seq[DataGenerator[_]] = Seq.empty

) extends RandomDataGenerator[(Seq[String],Seq[Seq[String]])] {


  /**
   * Return a Tuple, The Header Row, and the list of data as a sequence of strings
   * @return
   */
  override def generate(): (Seq[String],Seq[Seq[String]]) = {

    // if there is no dependencies, we can return the order of the generators as is
    val generatorsOrder = dependentOn.isEmpty match {
      case true => generators
      case false => dependencyOrdered
    }

    // the original generator order needs to be preserved
    // if the columns depend on each other they will get all mixed up
    // so we need to reorder the result to match the original incoming order

    // This magic below does
    // the original order is (c,a,b)
    // the "generating order" is (b,a,c)
    // so the results will be created in b,a,c but need to be returned as c,a,b order

    // (b,a,c) to (  (b,0),  (a,1),  (c,2)  )
    //         to (  (a,1),  (b,0),  (c,2)  ) // sorted on name
    // zip the original  ( (c,0), (a,1) (b,2) )
    //   and sort ( (a,1) (b,2), (c,0) ) on name
    //
    // now zip both (  ((a,1),(a,1)),  ((b,0),(b,2)),  ((c,2),(c,0))  )
    // now sort on first index and extract 2nd index
    //  (  ((b,0),(b,2)),  ((a,1),(a,1)),  ((c,2),(c,0))  ) --> (2,1,0)
    //
    //  (b,2)  (a,1)  (c,0)  ->  (c,a,b)

    val sortIndex = dependentOn.isEmpty match {
      case true => null
      case false => dependencyOrdered.zipWithIndex.sortBy(_._1.name).zip(
                           generators.zipWithIndex.sortBy(_._1.name)
                    ).sortBy(_._1._2).map(_._2._2)
    }

    val result = (0 until rows).toList.map { i =>
      val ctx:Context = new Context(i) // holds the values for this rods
      val row = for {
        generator <- generatorsOrder // this is the corrrect generation order, but it mucks up the order
        valGen = generator.generate(ctx,dependentOn)
      } yield {
        valGen.toString
      }

      dependentOn.isEmpty match {
        case true => row
        case false => row.zip(sortIndex).sortBy(_._2).map(_._1)
      }
    }
    (generators.map(_.name),result)
  }

}
