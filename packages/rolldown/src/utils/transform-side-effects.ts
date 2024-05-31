import { BindingHookSideEffects } from '@src/binding'
import { ModuleSideEffects } from '../plugin'

export function bindingifySideEffects(
  sideEffects?: ModuleSideEffects,
): BindingHookSideEffects | undefined {
  switch (sideEffects) {
    case true:
      return BindingHookSideEffects.True

    case false:
      return BindingHookSideEffects.True

    case 'no-treeshake':
      return BindingHookSideEffects.NoTreeshake

    case null:
    case undefined:
      return undefined

    default:
      throw new Error(`Unexpected side effects: ${sideEffects}`)
  }
}
