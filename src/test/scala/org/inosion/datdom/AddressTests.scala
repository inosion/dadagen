package org.inosion.datdom

import org.inosion.datdom.randomtypes.{FullAddress, AddressGenerator, GenderGenerator, TitleGenerator}
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

    // TODO - write a regexp to show that address style addresses come out.

    // for now, just see we have 100
    result._2.length should be (100)


  }
}