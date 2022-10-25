#[doc = "Register `mclkcfg` reader"]
pub struct R(crate::R<MCLKCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCLKCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCLKCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCLKCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `mclkcfg` writer"]
pub struct W(crate::W<MCLKCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCLKCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<MCLKCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCLKCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `asrc_mclk_freq_div_coe` reader - Frequency Division Coefficient"]
pub type ASRC_MCLK_FREQ_DIV_COE_R = crate::FieldReader<u8, ASRC_MCLK_FREQ_DIV_COE_A>;
#[doc = "Frequency Division Coefficient\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ASRC_MCLK_FREQ_DIV_COE_A {
    #[doc = "0: Reserved. No output."]
    R_ESERVED = 0,
    #[doc = "1: 1/1"]
    DIV1 = 1,
    #[doc = "2: 1/2"]
    DIV2 = 2,
    #[doc = "3: 1/4"]
    DIV4 = 3,
    #[doc = "4: 1/6"]
    DIV6 = 4,
    #[doc = "5: 1/8"]
    DIV8 = 5,
    #[doc = "6: 1/12"]
    DIV12 = 6,
    #[doc = "7: 1/16"]
    DIV16 = 7,
    #[doc = "8: 1/24"]
    DIV24 = 8,
    #[doc = "9: 1/32"]
    DIV32 = 9,
    #[doc = "10: 1/48"]
    DIV48 = 10,
    #[doc = "11: 1/64"]
    DIV64 = 11,
    #[doc = "12: 1/96"]
    DIV96 = 12,
    #[doc = "13: 1/128"]
    DIV128 = 13,
    #[doc = "14: 1/176"]
    DIV176 = 14,
    #[doc = "15: 1/192"]
    DIV192 = 15,
}
impl From<ASRC_MCLK_FREQ_DIV_COE_A> for u8 {
    #[inline(always)]
    fn from(variant: ASRC_MCLK_FREQ_DIV_COE_A) -> Self {
        variant as _
    }
}
impl ASRC_MCLK_FREQ_DIV_COE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASRC_MCLK_FREQ_DIV_COE_A {
        match self.bits {
            0 => ASRC_MCLK_FREQ_DIV_COE_A::R_ESERVED,
            1 => ASRC_MCLK_FREQ_DIV_COE_A::DIV1,
            2 => ASRC_MCLK_FREQ_DIV_COE_A::DIV2,
            3 => ASRC_MCLK_FREQ_DIV_COE_A::DIV4,
            4 => ASRC_MCLK_FREQ_DIV_COE_A::DIV6,
            5 => ASRC_MCLK_FREQ_DIV_COE_A::DIV8,
            6 => ASRC_MCLK_FREQ_DIV_COE_A::DIV12,
            7 => ASRC_MCLK_FREQ_DIV_COE_A::DIV16,
            8 => ASRC_MCLK_FREQ_DIV_COE_A::DIV24,
            9 => ASRC_MCLK_FREQ_DIV_COE_A::DIV32,
            10 => ASRC_MCLK_FREQ_DIV_COE_A::DIV48,
            11 => ASRC_MCLK_FREQ_DIV_COE_A::DIV64,
            12 => ASRC_MCLK_FREQ_DIV_COE_A::DIV96,
            13 => ASRC_MCLK_FREQ_DIV_COE_A::DIV128,
            14 => ASRC_MCLK_FREQ_DIV_COE_A::DIV176,
            15 => ASRC_MCLK_FREQ_DIV_COE_A::DIV192,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `R_ESERVED`"]
    #[inline(always)]
    pub fn is_r_eserved(&self) -> bool {
        *self == ASRC_MCLK_FREQ_DIV_COE_A::R_ESERVED
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == ASRC_MCLK_FREQ_DIV_COE_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == ASRC_MCLK_FREQ_DIV_COE_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == ASRC_MCLK_FREQ_DIV_COE_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == ASRC_MCLK_FREQ_DIV_COE_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == ASRC_MCLK_FREQ_DIV_COE_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == ASRC_MCLK_FREQ_DIV_COE_A::DIV12
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == ASRC_MCLK_FREQ_DIV_COE_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV24`"]
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == ASRC_MCLK_FREQ_DIV_COE_A::DIV24
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == ASRC_MCLK_FREQ_DIV_COE_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV48`"]
    #[inline(always)]
    pub fn is_div48(&self) -> bool {
        *self == ASRC_MCLK_FREQ_DIV_COE_A::DIV48
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == ASRC_MCLK_FREQ_DIV_COE_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV96`"]
    #[inline(always)]
    pub fn is_div96(&self) -> bool {
        *self == ASRC_MCLK_FREQ_DIV_COE_A::DIV96
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == ASRC_MCLK_FREQ_DIV_COE_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV176`"]
    #[inline(always)]
    pub fn is_div176(&self) -> bool {
        *self == ASRC_MCLK_FREQ_DIV_COE_A::DIV176
    }
    #[doc = "Checks if the value of the field is `DIV192`"]
    #[inline(always)]
    pub fn is_div192(&self) -> bool {
        *self == ASRC_MCLK_FREQ_DIV_COE_A::DIV192
    }
}
#[doc = "Field `asrc_mclk_freq_div_coe` writer - Frequency Division Coefficient"]
pub type ASRC_MCLK_FREQ_DIV_COE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, MCLKCFG_SPEC, u8, ASRC_MCLK_FREQ_DIV_COE_A, 4, O>;
impl<'a, const O: u8> ASRC_MCLK_FREQ_DIV_COE_W<'a, O> {
    #[doc = "Reserved. No output."]
    #[inline(always)]
    pub fn r_eserved(self) -> &'a mut W {
        self.variant(ASRC_MCLK_FREQ_DIV_COE_A::R_ESERVED)
    }
    #[doc = "1/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(ASRC_MCLK_FREQ_DIV_COE_A::DIV1)
    }
    #[doc = "1/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(ASRC_MCLK_FREQ_DIV_COE_A::DIV2)
    }
    #[doc = "1/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(ASRC_MCLK_FREQ_DIV_COE_A::DIV4)
    }
    #[doc = "1/6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(ASRC_MCLK_FREQ_DIV_COE_A::DIV6)
    }
    #[doc = "1/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(ASRC_MCLK_FREQ_DIV_COE_A::DIV8)
    }
    #[doc = "1/12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(ASRC_MCLK_FREQ_DIV_COE_A::DIV12)
    }
    #[doc = "1/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(ASRC_MCLK_FREQ_DIV_COE_A::DIV16)
    }
    #[doc = "1/24"]
    #[inline(always)]
    pub fn div24(self) -> &'a mut W {
        self.variant(ASRC_MCLK_FREQ_DIV_COE_A::DIV24)
    }
    #[doc = "1/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(ASRC_MCLK_FREQ_DIV_COE_A::DIV32)
    }
    #[doc = "1/48"]
    #[inline(always)]
    pub fn div48(self) -> &'a mut W {
        self.variant(ASRC_MCLK_FREQ_DIV_COE_A::DIV48)
    }
    #[doc = "1/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(ASRC_MCLK_FREQ_DIV_COE_A::DIV64)
    }
    #[doc = "1/96"]
    #[inline(always)]
    pub fn div96(self) -> &'a mut W {
        self.variant(ASRC_MCLK_FREQ_DIV_COE_A::DIV96)
    }
    #[doc = "1/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(ASRC_MCLK_FREQ_DIV_COE_A::DIV128)
    }
    #[doc = "1/176"]
    #[inline(always)]
    pub fn div176(self) -> &'a mut W {
        self.variant(ASRC_MCLK_FREQ_DIV_COE_A::DIV176)
    }
    #[doc = "1/192"]
    #[inline(always)]
    pub fn div192(self) -> &'a mut W {
        self.variant(ASRC_MCLK_FREQ_DIV_COE_A::DIV192)
    }
}
#[doc = "Field `asrc_mclk_gate` reader - ASRC Clock Gate Enable Control"]
pub type ASRC_MCLK_GATE_R = crate::BitReader<ASRC_MCLK_GATE_A>;
#[doc = "ASRC Clock Gate Enable Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ASRC_MCLK_GATE_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<ASRC_MCLK_GATE_A> for bool {
    #[inline(always)]
    fn from(variant: ASRC_MCLK_GATE_A) -> Self {
        variant as u8 != 0
    }
}
impl ASRC_MCLK_GATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ASRC_MCLK_GATE_A {
        match self.bits {
            false => ASRC_MCLK_GATE_A::DISABLE,
            true => ASRC_MCLK_GATE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ASRC_MCLK_GATE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ASRC_MCLK_GATE_A::ENABLE
    }
}
#[doc = "Field `asrc_mclk_gate` writer - ASRC Clock Gate Enable Control"]
pub type ASRC_MCLK_GATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MCLKCFG_SPEC, ASRC_MCLK_GATE_A, O>;
impl<'a, const O: u8> ASRC_MCLK_GATE_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ASRC_MCLK_GATE_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ASRC_MCLK_GATE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bits 0:3 - Frequency Division Coefficient"]
    #[inline(always)]
    pub fn asrc_mclk_freq_div_coe(&self) -> ASRC_MCLK_FREQ_DIV_COE_R {
        ASRC_MCLK_FREQ_DIV_COE_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 16 - ASRC Clock Gate Enable Control"]
    #[inline(always)]
    pub fn asrc_mclk_gate(&self) -> ASRC_MCLK_GATE_R {
        ASRC_MCLK_GATE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Frequency Division Coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn asrc_mclk_freq_div_coe(&mut self) -> ASRC_MCLK_FREQ_DIV_COE_W<0> {
        ASRC_MCLK_FREQ_DIV_COE_W::new(self)
    }
    #[doc = "Bit 16 - ASRC Clock Gate Enable Control"]
    #[inline(always)]
    #[must_use]
    pub fn asrc_mclk_gate(&mut self) -> ASRC_MCLK_GATE_W<16> {
        ASRC_MCLK_GATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ASRC MCLK Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mclkcfg](index.html) module"]
pub struct MCLKCFG_SPEC;
impl crate::RegisterSpec for MCLKCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mclkcfg::R](R) reader structure"]
impl crate::Readable for MCLKCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mclkcfg::W](W) writer structure"]
impl crate::Writable for MCLKCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets mclkcfg to value 0"]
impl crate::Resettable for MCLKCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
