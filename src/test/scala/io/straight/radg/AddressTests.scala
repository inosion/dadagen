package io.straight.radg

import io.straight.radg.randomtypes.{FullAddress, AddressGenerator, GenderGenerator, TitleGenerator}
import org.scalatest.{Matchers, FlatSpec}

/**
 * @author rbuckland
 */
class AddressTests  extends FlatSpec with Matchers {

  "generating an address in full" should "look good :-)" in {
    val ramondom = SeqOfSeqOfStringsGenerator(100,List(
      AddressGenerator("fulllineaddress",FullAddress)
    ))

    val result = ramondom.generate()
    for (row <- result._2) {
      println(row)
    }
  }
}
