use quote::Tokens;

use crate::errors::*;

/// Generates generic bit munging code
pub fn render() -> Result<Vec<Tokens>> {
    let mut code = vec![];

    code.push(quote! {
        /// Single bit read access proxy
        trait BitR {
            /// Returns `true` if the bit is clear (0)
            #[inline]
            fn bit_is_clear(&self) -> bool {
                !self.bit()
            }
            /// Returns `true` if the bit is set (1)
            #[inline]
            fn bit_is_set(&self) -> bool {
                self.bit()
            }

            /// Returns the current state of the bit as boolean
            fn bit(&self) -> bool;
        }

        /// Single bit write access proxy
        trait BitW<'a, W> {
            /// Sets the field bit
            #[inline]
            fn set_bit(self) -> &'a mut W
            where
                Self: core::marker::Sized,
            {
                self.bit(true)
            }
            /// Clears the field bit
            #[inline]
            fn clear_bit(self) -> &'a mut W
            where
                Self: core::marker::Sized,
            {
                self.bit(false)
            }

            /// Writes raw bit(s) to the field
            fn bit(self, value: bool) -> &'a mut W
            where
                Self: core::marker::Sized;
        }

        /// Wiggle in the specified value into the given bits with mask and the offset and return
        /// the new value
        #[inline]
        const fn set_bits(bits: u32, mask: u32, offset: u8, value: u32) -> u32 {
            let mut bits = bits;
            bits &= !((mask as u32) << offset);
            bits |= ((value & mask) as u32) << offset;
            bits
        }
    });

    Ok(code)
}
