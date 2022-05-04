#[doc = "Register `GP_FIFO_INTC` reader"]
pub struct R(crate::R<GP_FIFO_INTC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GP_FIFO_INTC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GP_FIFO_INTC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GP_FIFO_INTC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GP_FIFO_INTC` writer"]
pub struct W(crate::W<GP_FIFO_INTC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GP_FIFO_INTC_SPEC>;
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
impl From<crate::W<GP_FIFO_INTC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GP_FIFO_INTC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ADC FIFO Date DRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_DATA_DRQ_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<FIFO_DATA_DRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_DATA_DRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_DATA_DRQ_EN` reader - ADC FIFO Date DRQ Enable"]
pub struct FIFO_DATA_DRQ_EN_R(crate::FieldReader<bool>);
impl FIFO_DATA_DRQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_DATA_DRQ_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_DATA_DRQ_EN_A {
        match self.bits {
            false => FIFO_DATA_DRQ_EN_A::DISABLE,
            true => FIFO_DATA_DRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FIFO_DATA_DRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FIFO_DATA_DRQ_EN_A::ENABLE
    }
}
impl core::ops::Deref for FIFO_DATA_DRQ_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_DATA_DRQ_EN` writer - ADC FIFO Date DRQ Enable"]
pub struct FIFO_DATA_DRQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_DATA_DRQ_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFO_DATA_DRQ_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FIFO_DATA_DRQ_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FIFO_DATA_DRQ_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u32 & 1) << 18);
        self.w
    }
}
#[doc = "ADC FIFO Overrun IRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_OVERRUN_IRQ_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<FIFO_OVERRUN_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_OVERRUN_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_OVERRUN_IRQ_EN` reader - ADC FIFO Overrun IRQ Enable"]
pub struct FIFO_OVERRUN_IRQ_EN_R(crate::FieldReader<bool>);
impl FIFO_OVERRUN_IRQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_OVERRUN_IRQ_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_OVERRUN_IRQ_EN_A {
        match self.bits {
            false => FIFO_OVERRUN_IRQ_EN_A::DISABLE,
            true => FIFO_OVERRUN_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FIFO_OVERRUN_IRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FIFO_OVERRUN_IRQ_EN_A::ENABLE
    }
}
impl core::ops::Deref for FIFO_OVERRUN_IRQ_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_OVERRUN_IRQ_EN` writer - ADC FIFO Overrun IRQ Enable"]
pub struct FIFO_OVERRUN_IRQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_OVERRUN_IRQ_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFO_OVERRUN_IRQ_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FIFO_OVERRUN_IRQ_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FIFO_OVERRUN_IRQ_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
#[doc = "ADC FIFO Data Available IRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FIFO_DATA_IRQ_EN_A {
    #[doc = "0: Disable"]
    DISABLE = 0,
    #[doc = "1: Enable"]
    ENABLE = 1,
}
impl From<FIFO_DATA_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: FIFO_DATA_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFO_DATA_IRQ_EN` reader - ADC FIFO Data Available IRQ Enable"]
pub struct FIFO_DATA_IRQ_EN_R(crate::FieldReader<bool>);
impl FIFO_DATA_IRQ_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_DATA_IRQ_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIFO_DATA_IRQ_EN_A {
        match self.bits {
            false => FIFO_DATA_IRQ_EN_A::DISABLE,
            true => FIFO_DATA_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == FIFO_DATA_IRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        **self == FIFO_DATA_IRQ_EN_A::ENABLE
    }
}
impl core::ops::Deref for FIFO_DATA_IRQ_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_DATA_IRQ_EN` writer - ADC FIFO Data Available IRQ Enable"]
pub struct FIFO_DATA_IRQ_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_DATA_IRQ_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FIFO_DATA_IRQ_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FIFO_DATA_IRQ_EN_A::DISABLE)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FIFO_DATA_IRQ_EN_A::ENABLE)
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `FIFO_TRIG_LEVEL` reader - Interrupt trigger level for ADC\n\nTrigger Level = TXTL + 1"]
pub struct FIFO_TRIG_LEVEL_R(crate::FieldReader<u8>);
impl FIFO_TRIG_LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FIFO_TRIG_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_TRIG_LEVEL_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_TRIG_LEVEL` writer - Interrupt trigger level for ADC\n\nTrigger Level = TXTL + 1"]
pub struct FIFO_TRIG_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_TRIG_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `FIFO_FLUSH` reader - ADC FIFO Flush\n\nWrite 1 to flush TX FIFO, clear automatically to 0."]
pub struct FIFO_FLUSH_R(crate::FieldReader<bool>);
impl FIFO_FLUSH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_FLUSH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_FLUSH_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FIFO_FLUSH` writer - ADC FIFO Flush\n\nWrite 1 to flush TX FIFO, clear automatically to 0."]
pub struct FIFO_FLUSH_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_FLUSH_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 18 - ADC FIFO Date DRQ Enable"]
    #[inline(always)]
    pub fn fifo_data_drq_en(&self) -> FIFO_DATA_DRQ_EN_R {
        FIFO_DATA_DRQ_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC FIFO Overrun IRQ Enable"]
    #[inline(always)]
    pub fn fifo_overrun_irq_en(&self) -> FIFO_OVERRUN_IRQ_EN_R {
        FIFO_OVERRUN_IRQ_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC FIFO Data Available IRQ Enable"]
    #[inline(always)]
    pub fn fifo_data_irq_en(&self) -> FIFO_DATA_IRQ_EN_R {
        FIFO_DATA_IRQ_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Interrupt trigger level for ADC\n\nTrigger Level = TXTL + 1"]
    #[inline(always)]
    pub fn fifo_trig_level(&self) -> FIFO_TRIG_LEVEL_R {
        FIFO_TRIG_LEVEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 4 - ADC FIFO Flush\n\nWrite 1 to flush TX FIFO, clear automatically to 0."]
    #[inline(always)]
    pub fn fifo_flush(&self) -> FIFO_FLUSH_R {
        FIFO_FLUSH_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - ADC FIFO Date DRQ Enable"]
    #[inline(always)]
    pub fn fifo_data_drq_en(&mut self) -> FIFO_DATA_DRQ_EN_W {
        FIFO_DATA_DRQ_EN_W { w: self }
    }
    #[doc = "Bit 17 - ADC FIFO Overrun IRQ Enable"]
    #[inline(always)]
    pub fn fifo_overrun_irq_en(&mut self) -> FIFO_OVERRUN_IRQ_EN_W {
        FIFO_OVERRUN_IRQ_EN_W { w: self }
    }
    #[doc = "Bit 16 - ADC FIFO Data Available IRQ Enable"]
    #[inline(always)]
    pub fn fifo_data_irq_en(&mut self) -> FIFO_DATA_IRQ_EN_W {
        FIFO_DATA_IRQ_EN_W { w: self }
    }
    #[doc = "Bits 8:13 - Interrupt trigger level for ADC\n\nTrigger Level = TXTL + 1"]
    #[inline(always)]
    pub fn fifo_trig_level(&mut self) -> FIFO_TRIG_LEVEL_W {
        FIFO_TRIG_LEVEL_W { w: self }
    }
    #[doc = "Bit 4 - ADC FIFO Flush\n\nWrite 1 to flush TX FIFO, clear automatically to 0."]
    #[inline(always)]
    pub fn fifo_flush(&mut self) -> FIFO_FLUSH_W {
        FIFO_FLUSH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPADC FIFO Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gp_fifo_intc](index.html) module"]
pub struct GP_FIFO_INTC_SPEC;
impl crate::RegisterSpec for GP_FIFO_INTC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gp_fifo_intc::R](R) reader structure"]
impl crate::Readable for GP_FIFO_INTC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gp_fifo_intc::W](W) writer structure"]
impl crate::Writable for GP_FIFO_INTC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GP_FIFO_INTC to value 0x1f00"]
impl crate::Resettable for GP_FIFO_INTC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f00
    }
}
