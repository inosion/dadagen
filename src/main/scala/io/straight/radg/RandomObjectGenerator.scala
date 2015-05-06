package io.straight.radg

import io.straight.radg.randomtypes.DataGenerator

/**
 * @author rbuckland
 */
case class RandomObjectGenerator(
                             rows:Int,
                             // the rowConfig is an ordering
                             generators:Seq[DataGenerator[_]] = Seq.empty

                             ) extends RandomDataGenerator[Seq[AnyRef]] {
  override def generate(): Seq[AnyRef] = ???
}


object ObjectGeneratorBuilder {

  //def from(manifest:Manifest):Seq[DataGenerator[_]] = ???

}