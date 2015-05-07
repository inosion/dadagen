package org.inosion.datdom

import org.inosion.datdom.randomtypes.DataGenerator

/**
 * @author rbuckland
 */
case class RandomObjectGenerator(
                             rows:Int,
                             // the rowConfig is an ordering
                             generators:List[DataGenerator[_]] = List.empty

                             ) extends RandomDataGenerator[List[AnyRef]] {
  override def generate(): List[AnyRef] = ???
}


object ObjectGeneratorBuilder {

  //def from(manifest:Manifest):Seq[DataGenerator[_]] = ???

}
