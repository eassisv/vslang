export interface QueueNode {
  value: any,
  next: QueueNode | null
  ttl: number
}

export default class Queue {
  size: number
  head: QueueNode | null
  tail: QueueNode | null

  constructor() {
    this.head = null
    this.tail = null
    this.size = 0
  }

  push(value: any, ttl: number) {
    const node: QueueNode = {
      value,
      ttl,
      next: null
    }

    if (this.size) {
      this.tail!.next = this.tail = node
    } else {
      this.head = this.tail = node
    }
    this.size++
  }

  pop() {
    if (this.size) {
      this.head = this.head!.next
      this.size--
    }
  }

  peek(): QueueNode | null {
    return this.head
  }
}

