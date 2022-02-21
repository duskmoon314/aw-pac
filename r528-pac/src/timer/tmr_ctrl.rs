#[doc = "Register `tmr%s_ctrl` reader"]
pub struct R(crate::R<TMR_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TMR_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TMR_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TMR_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tmr%s_ctrl` writer"]
pub struct W(crate::W<TMR_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TMR_CTRL_SPEC>;
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
impl From<crate::W<TMR_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TMR_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR_MODE_A {
    #[doc = "0: `0`"]
    PERIODIC = 0,
    #[doc = "1: `1`"]
    SINGLE_COUNTING = 1,
}
impl From<TMR_MODE_A> for bool {
    #[inline(always)]
    fn from(variant: TMR_MODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `tmr_mode` reader - "]
pub struct TMR_MODE_R(crate::FieldReader<bool, TMR_MODE_A>);
impl TMR_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TMR_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR_MODE_A {
        match self.bits {
            false => TMR_MODE_A::PERIODIC,
            true => TMR_MODE_A::SINGLE_COUNTING,
        }
    }
    #[doc = "Checks if the value of the field is `PERIODIC`"]
    #[inline(always)]
    pub fn is_periodic(&self) -> bool {
        **self == TMR_MODE_A::PERIODIC
    }
    #[doc = "Checks if the value of the field is `SINGLE_COUNTING`"]
    #[inline(always)]
    pub fn is_single_counting(&self) -> bool {
        **self == TMR_MODE_A::SINGLE_COUNTING
    }
}
impl core::ops::Deref for TMR_MODE_R {
    type Target = crate::FieldReader<bool, TMR_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tmr_mode` writer - "]
pub struct TMR_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR_MODE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn periodic(self) -> &'a mut W {
        self.variant(TMR_MODE_A::PERIODIC)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn single_counting(self) -> &'a mut W {
        self.variant(TMR_MODE_A::SINGLE_COUNTING)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMR_CLK_PRES_A {
    #[doc = "0: `0`"]
    P1 = 0,
    #[doc = "1: `1`"]
    P2 = 1,
    #[doc = "2: `10`"]
    P4 = 2,
    #[doc = "3: `11`"]
    P8 = 3,
    #[doc = "4: `100`"]
    P16 = 4,
    #[doc = "5: `101`"]
    P32 = 5,
    #[doc = "6: `110`"]
    P64 = 6,
    #[doc = "7: `111`"]
    P128 = 7,
}
impl From<TMR_CLK_PRES_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR_CLK_PRES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `tmr_clk_pres` reader - "]
pub struct TMR_CLK_PRES_R(crate::FieldReader<u8, TMR_CLK_PRES_A>);
impl TMR_CLK_PRES_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TMR_CLK_PRES_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR_CLK_PRES_A {
        match self.bits {
            0 => TMR_CLK_PRES_A::P1,
            1 => TMR_CLK_PRES_A::P2,
            2 => TMR_CLK_PRES_A::P4,
            3 => TMR_CLK_PRES_A::P8,
            4 => TMR_CLK_PRES_A::P16,
            5 => TMR_CLK_PRES_A::P32,
            6 => TMR_CLK_PRES_A::P64,
            7 => TMR_CLK_PRES_A::P128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `P1`"]
    #[inline(always)]
    pub fn is_p1(&self) -> bool {
        **self == TMR_CLK_PRES_A::P1
    }
    #[doc = "Checks if the value of the field is `P2`"]
    #[inline(always)]
    pub fn is_p2(&self) -> bool {
        **self == TMR_CLK_PRES_A::P2
    }
    #[doc = "Checks if the value of the field is `P4`"]
    #[inline(always)]
    pub fn is_p4(&self) -> bool {
        **self == TMR_CLK_PRES_A::P4
    }
    #[doc = "Checks if the value of the field is `P8`"]
    #[inline(always)]
    pub fn is_p8(&self) -> bool {
        **self == TMR_CLK_PRES_A::P8
    }
    #[doc = "Checks if the value of the field is `P16`"]
    #[inline(always)]
    pub fn is_p16(&self) -> bool {
        **self == TMR_CLK_PRES_A::P16
    }
    #[doc = "Checks if the value of the field is `P32`"]
    #[inline(always)]
    pub fn is_p32(&self) -> bool {
        **self == TMR_CLK_PRES_A::P32
    }
    #[doc = "Checks if the value of the field is `P64`"]
    #[inline(always)]
    pub fn is_p64(&self) -> bool {
        **self == TMR_CLK_PRES_A::P64
    }
    #[doc = "Checks if the value of the field is `P128`"]
    #[inline(always)]
    pub fn is_p128(&self) -> bool {
        **self == TMR_CLK_PRES_A::P128
    }
}
impl core::ops::Deref for TMR_CLK_PRES_R {
    type Target = crate::FieldReader<u8, TMR_CLK_PRES_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tmr_clk_pres` writer - "]
pub struct TMR_CLK_PRES_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR_CLK_PRES_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR_CLK_PRES_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn p1(self) -> &'a mut W {
        self.variant(TMR_CLK_PRES_A::P1)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn p2(self) -> &'a mut W {
        self.variant(TMR_CLK_PRES_A::P2)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn p4(self) -> &'a mut W {
        self.variant(TMR_CLK_PRES_A::P4)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn p8(self) -> &'a mut W {
        self.variant(TMR_CLK_PRES_A::P8)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn p16(self) -> &'a mut W {
        self.variant(TMR_CLK_PRES_A::P16)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn p32(self) -> &'a mut W {
        self.variant(TMR_CLK_PRES_A::P32)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn p64(self) -> &'a mut W {
        self.variant(TMR_CLK_PRES_A::P64)
    }
    #[doc = "`111`"]
    #[inline(always)]
    pub fn p128(self) -> &'a mut W {
        self.variant(TMR_CLK_PRES_A::P128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TMR_CLK_SRC_A {
    #[doc = "0: `0`"]
    LOSC = 0,
    #[doc = "1: `1`"]
    OSC24_M = 1,
}
impl From<TMR_CLK_SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: TMR_CLK_SRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `tmr_clk_src` reader - "]
pub struct TMR_CLK_SRC_R(crate::FieldReader<u8, TMR_CLK_SRC_A>);
impl TMR_CLK_SRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TMR_CLK_SRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TMR_CLK_SRC_A> {
        match self.bits {
            0 => Some(TMR_CLK_SRC_A::LOSC),
            1 => Some(TMR_CLK_SRC_A::OSC24_M),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOSC`"]
    #[inline(always)]
    pub fn is_losc(&self) -> bool {
        **self == TMR_CLK_SRC_A::LOSC
    }
    #[doc = "Checks if the value of the field is `OSC24_M`"]
    #[inline(always)]
    pub fn is_osc24_m(&self) -> bool {
        **self == TMR_CLK_SRC_A::OSC24_M
    }
}
impl core::ops::Deref for TMR_CLK_SRC_R {
    type Target = crate::FieldReader<u8, TMR_CLK_SRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tmr_clk_src` writer - "]
pub struct TMR_CLK_SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR_CLK_SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR_CLK_SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn losc(self) -> &'a mut W {
        self.variant(TMR_CLK_SRC_A::LOSC)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn osc24_m(self) -> &'a mut W {
        self.variant(TMR_CLK_SRC_A::OSC24_M)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR_RELOAD_A {
    #[doc = "0: `0`"]
    NO_EFFECT = 0,
    #[doc = "1: `1`"]
    RELOAD = 1,
}
impl From<TMR_RELOAD_A> for bool {
    #[inline(always)]
    fn from(variant: TMR_RELOAD_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `tmr_reload` reader - "]
pub struct TMR_RELOAD_R(crate::FieldReader<bool, TMR_RELOAD_A>);
impl TMR_RELOAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TMR_RELOAD_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR_RELOAD_A {
        match self.bits {
            false => TMR_RELOAD_A::NO_EFFECT,
            true => TMR_RELOAD_A::RELOAD,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == TMR_RELOAD_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `RELOAD`"]
    #[inline(always)]
    pub fn is_reload(&self) -> bool {
        **self == TMR_RELOAD_A::RELOAD
    }
}
impl core::ops::Deref for TMR_RELOAD_R {
    type Target = crate::FieldReader<bool, TMR_RELOAD_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tmr_reload` writer - "]
pub struct TMR_RELOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR_RELOAD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR_RELOAD_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TMR_RELOAD_A::NO_EFFECT)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn reload(self) -> &'a mut W {
        self.variant(TMR_RELOAD_A::RELOAD)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TMR_EN_A {
    #[doc = "0: `0`"]
    STOP_PAUSE = 0,
    #[doc = "1: `1`"]
    START = 1,
}
impl From<TMR_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TMR_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `tmr_en` reader - "]
pub struct TMR_EN_R(crate::FieldReader<bool, TMR_EN_A>);
impl TMR_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TMR_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TMR_EN_A {
        match self.bits {
            false => TMR_EN_A::STOP_PAUSE,
            true => TMR_EN_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `STOP_PAUSE`"]
    #[inline(always)]
    pub fn is_stop_pause(&self) -> bool {
        **self == TMR_EN_A::STOP_PAUSE
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        **self == TMR_EN_A::START
    }
}
impl core::ops::Deref for TMR_EN_R {
    type Target = crate::FieldReader<bool, TMR_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tmr_en` writer - "]
pub struct TMR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TMR_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn stop_pause(self) -> &'a mut W {
        self.variant(TMR_EN_A::STOP_PAUSE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(TMR_EN_A::START)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tmr_mode(&self) -> TMR_MODE_R {
        TMR_MODE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn tmr_clk_pres(&self) -> TMR_CLK_PRES_R {
        TMR_CLK_PRES_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tmr_clk_src(&self) -> TMR_CLK_SRC_R {
        TMR_CLK_SRC_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tmr_reload(&self) -> TMR_RELOAD_R {
        TMR_RELOAD_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tmr_en(&self) -> TMR_EN_R {
        TMR_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn tmr_mode(&mut self) -> TMR_MODE_W {
        TMR_MODE_W { w: self }
    }
    #[doc = "Bits 4:6"]
    #[inline(always)]
    pub fn tmr_clk_pres(&mut self) -> TMR_CLK_PRES_W {
        TMR_CLK_PRES_W { w: self }
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn tmr_clk_src(&mut self) -> TMR_CLK_SRC_W {
        TMR_CLK_SRC_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tmr_reload(&mut self) -> TMR_RELOAD_W {
        TMR_RELOAD_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tmr_en(&mut self) -> TMR_EN_W {
        TMR_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer IRQ Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tmr_ctrl](index.html) module"]
pub struct TMR_CTRL_SPEC;
impl crate::RegisterSpec for TMR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tmr_ctrl::R](R) reader structure"]
impl crate::Readable for TMR_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tmr_ctrl::W](W) writer structure"]
impl crate::Writable for TMR_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets tmr%s_ctrl to value 0"]
impl crate::Resettable for TMR_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
