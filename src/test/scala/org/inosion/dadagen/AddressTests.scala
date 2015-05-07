package org.inosion.dadagen

import org.inosion.dadagen.randomtypes.{FullAddress, AddressGenerator, GenderGenerator, TitleGenerator}
import org.scalatest.{Matchers, FlatSpec}

/**
 * @author rbuckland
 */
class AddressTests  extends FlatSpec with Matchers {

  "generating an address in full" should "look good :-)" in {
    val ramondom = SeqOfSeqOfStringsGenerator(List(
      AddressGenerator("fulllineaddress",FullAddress)
    ))


    val result = ramondom.generateAll(100)

    // TODO - write a regexp to show that address style addresses come out.

    // for now, just see we have 100
    result._2.length should be (100)


  }
}
