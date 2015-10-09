package org.inosion.dadagen.auto

trait BaseGenerator[T] {
  def build(obj: T): IteratingRandomGenerator[T]
}
