import test from 'ava'

import { plus100, plus100FromZig } from '../index.js'

test('sync function from rust code', (t) => {
  const fixture = 42
  t.is(plus100(fixture), fixture + 100)
})

test('sync function from zig code', (t) => {
  const fixture = 42
  t.is(plus100FromZig(fixture), fixture + 100)
})
