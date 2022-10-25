#[doc = "Register `tp_int_fifo_ctrl` reader"]
pub struct R(crate::R<TP_INT_FIFO_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TP_INT_FIFO_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TP_INT_FIFO_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TP_INT_FIFO_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `tp_int_fifo_ctrl` writer"]
pub struct W(crate::W<TP_INT_FIFO_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TP_INT_FIFO_CTRL_SPEC>;
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
impl From<crate::W<TP_INT_FIFO_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TP_INT_FIFO_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tp_down_irq_en` reader - TP First Touch (Stylus DOWN) IRQ Enable"]
pub type TP_DOWN_IRQ_EN_R = crate::BitReader<TP_DOWN_IRQ_EN_A>;
#[doc = "TP First Touch (Stylus DOWN) IRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TP_DOWN_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TP_DOWN_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TP_DOWN_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TP_DOWN_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP_DOWN_IRQ_EN_A {
        match self.bits {
            false => TP_DOWN_IRQ_EN_A::DISABLE,
            true => TP_DOWN_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TP_DOWN_IRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TP_DOWN_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `tp_down_irq_en` writer - TP First Touch (Stylus DOWN) IRQ Enable"]
pub type TP_DOWN_IRQ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TP_INT_FIFO_CTRL_SPEC, TP_DOWN_IRQ_EN_A, O>;
impl<'a, const O: u8> TP_DOWN_IRQ_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TP_DOWN_IRQ_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TP_DOWN_IRQ_EN_A::ENABLE)
    }
}
#[doc = "Field `tp_up_irq_en` reader - TP Last Touch (Stylus UP) IRQ Enable"]
pub type TP_UP_IRQ_EN_R = crate::BitReader<TP_UP_IRQ_EN_A>;
#[doc = "TP Last Touch (Stylus UP) IRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TP_UP_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TP_UP_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TP_UP_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TP_UP_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP_UP_IRQ_EN_A {
        match self.bits {
            false => TP_UP_IRQ_EN_A::DISABLE,
            true => TP_UP_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TP_UP_IRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TP_UP_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `tp_up_irq_en` writer - TP Last Touch (Stylus UP) IRQ Enable"]
pub type TP_UP_IRQ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TP_INT_FIFO_CTRL_SPEC, TP_UP_IRQ_EN_A, O>;
impl<'a, const O: u8> TP_UP_IRQ_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TP_UP_IRQ_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TP_UP_IRQ_EN_A::ENABLE)
    }
}
#[doc = "Field `tp_fifo_flush` reader - TP FIFO Flush"]
pub type TP_FIFO_FLUSH_R = crate::BitReader<TP_FIFO_FLUSH_A>;
#[doc = "TP FIFO Flush\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TP_FIFO_FLUSH_A {
    #[doc = "1: `1`"]
    FLUSH = 1,
}
impl From<TP_FIFO_FLUSH_A> for bool {
    #[inline(always)]
    fn from(variant: TP_FIFO_FLUSH_A) -> Self {
        variant as u8 != 0
    }
}
impl TP_FIFO_FLUSH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TP_FIFO_FLUSH_A> {
        match self.bits {
            true => Some(TP_FIFO_FLUSH_A::FLUSH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FLUSH`"]
    #[inline(always)]
    pub fn is_flush(&self) -> bool {
        *self == TP_FIFO_FLUSH_A::FLUSH
    }
}
#[doc = "Field `tp_fifo_flush` writer - TP FIFO Flush"]
pub type TP_FIFO_FLUSH_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TP_INT_FIFO_CTRL_SPEC, TP_FIFO_FLUSH_A, O>;
impl<'a, const O: u8> TP_FIFO_FLUSH_W<'a, O> {
    #[doc = "`1`"]
    #[inline(always)]
    pub fn flush(self) -> &'a mut W {
        self.variant(TP_FIFO_FLUSH_A::FLUSH)
    }
}
#[doc = "Field `tp_data_erq_en` reader - TP FIFO Data Available DRQ Enable"]
pub type TP_DATA_ERQ_EN_R = crate::BitReader<TP_DATA_ERQ_EN_A>;
#[doc = "TP FIFO Data Available DRQ Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TP_DATA_ERQ_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TP_DATA_ERQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TP_DATA_ERQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TP_DATA_ERQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP_DATA_ERQ_EN_A {
        match self.bits {
            false => TP_DATA_ERQ_EN_A::DISABLE,
            true => TP_DATA_ERQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TP_DATA_ERQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TP_DATA_ERQ_EN_A::ENABLE
    }
}
#[doc = "Field `tp_data_erq_en` writer - TP FIFO Data Available DRQ Enable"]
pub type TP_DATA_ERQ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TP_INT_FIFO_CTRL_SPEC, TP_DATA_ERQ_EN_A, O>;
impl<'a, const O: u8> TP_DATA_ERQ_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TP_DATA_ERQ_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TP_DATA_ERQ_EN_A::ENABLE)
    }
}
#[doc = "Field `tp_fifo_trig_level` reader - TP FIFO Data Available Trigger Level"]
pub type TP_FIFO_TRIG_LEVEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `tp_fifo_trig_level` writer - TP FIFO Data Available Trigger Level"]
pub type TP_FIFO_TRIG_LEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TP_INT_FIFO_CTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `tp_data_xy_change` reader - TP FIFO XY Data Interchange Function Select"]
pub type TP_DATA_XY_CHANGE_R = crate::BitReader<TP_DATA_XY_CHANGE_A>;
#[doc = "TP FIFO XY Data Interchange Function Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TP_DATA_XY_CHANGE_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TP_DATA_XY_CHANGE_A> for bool {
    #[inline(always)]
    fn from(variant: TP_DATA_XY_CHANGE_A) -> Self {
        variant as u8 != 0
    }
}
impl TP_DATA_XY_CHANGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP_DATA_XY_CHANGE_A {
        match self.bits {
            false => TP_DATA_XY_CHANGE_A::DISABLE,
            true => TP_DATA_XY_CHANGE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TP_DATA_XY_CHANGE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TP_DATA_XY_CHANGE_A::ENABLE
    }
}
#[doc = "Field `tp_data_xy_change` writer - TP FIFO XY Data Interchange Function Select"]
pub type TP_DATA_XY_CHANGE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TP_INT_FIFO_CTRL_SPEC, TP_DATA_XY_CHANGE_A, O>;
impl<'a, const O: u8> TP_DATA_XY_CHANGE_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TP_DATA_XY_CHANGE_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TP_DATA_XY_CHANGE_A::ENABLE)
    }
}
#[doc = "Field `tp_data_irq_en` reader - TP FIFO Data Interrupt Enable"]
pub type TP_DATA_IRQ_EN_R = crate::BitReader<TP_DATA_IRQ_EN_A>;
#[doc = "TP FIFO Data Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TP_DATA_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TP_DATA_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TP_DATA_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TP_DATA_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP_DATA_IRQ_EN_A {
        match self.bits {
            false => TP_DATA_IRQ_EN_A::DISABLE,
            true => TP_DATA_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TP_DATA_IRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TP_DATA_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `tp_data_irq_en` writer - TP FIFO Data Interrupt Enable"]
pub type TP_DATA_IRQ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TP_INT_FIFO_CTRL_SPEC, TP_DATA_IRQ_EN_A, O>;
impl<'a, const O: u8> TP_DATA_IRQ_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TP_DATA_IRQ_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TP_DATA_IRQ_EN_A::ENABLE)
    }
}
#[doc = "Field `tp_overrun_irq_en` reader - TP FIFO Overrun Interrupt Enable"]
pub type TP_OVERRUN_IRQ_EN_R = crate::BitReader<TP_OVERRUN_IRQ_EN_A>;
#[doc = "TP FIFO Overrun Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TP_OVERRUN_IRQ_EN_A {
    #[doc = "0: `0`"]
    DISABLE = 0,
    #[doc = "1: `1`"]
    ENABLE = 1,
}
impl From<TP_OVERRUN_IRQ_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TP_OVERRUN_IRQ_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TP_OVERRUN_IRQ_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TP_OVERRUN_IRQ_EN_A {
        match self.bits {
            false => TP_OVERRUN_IRQ_EN_A::DISABLE,
            true => TP_OVERRUN_IRQ_EN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TP_OVERRUN_IRQ_EN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == TP_OVERRUN_IRQ_EN_A::ENABLE
    }
}
#[doc = "Field `tp_overrun_irq_en` writer - TP FIFO Overrun Interrupt Enable"]
pub type TP_OVERRUN_IRQ_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TP_INT_FIFO_CTRL_SPEC, TP_OVERRUN_IRQ_EN_A, O>;
impl<'a, const O: u8> TP_OVERRUN_IRQ_EN_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TP_OVERRUN_IRQ_EN_A::DISABLE)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(TP_OVERRUN_IRQ_EN_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - TP First Touch (Stylus DOWN) IRQ Enable"]
    #[inline(always)]
    pub fn tp_down_irq_en(&self) -> TP_DOWN_IRQ_EN_R {
        TP_DOWN_IRQ_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TP Last Touch (Stylus UP) IRQ Enable"]
    #[inline(always)]
    pub fn tp_up_irq_en(&self) -> TP_UP_IRQ_EN_R {
        TP_UP_IRQ_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - TP FIFO Flush"]
    #[inline(always)]
    pub fn tp_fifo_flush(&self) -> TP_FIFO_FLUSH_R {
        TP_FIFO_FLUSH_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - TP FIFO Data Available DRQ Enable"]
    #[inline(always)]
    pub fn tp_data_erq_en(&self) -> TP_DATA_ERQ_EN_R {
        TP_DATA_ERQ_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - TP FIFO Data Available Trigger Level"]
    #[inline(always)]
    pub fn tp_fifo_trig_level(&self) -> TP_FIFO_TRIG_LEVEL_R {
        TP_FIFO_TRIG_LEVEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - TP FIFO XY Data Interchange Function Select"]
    #[inline(always)]
    pub fn tp_data_xy_change(&self) -> TP_DATA_XY_CHANGE_R {
        TP_DATA_XY_CHANGE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - TP FIFO Data Interrupt Enable"]
    #[inline(always)]
    pub fn tp_data_irq_en(&self) -> TP_DATA_IRQ_EN_R {
        TP_DATA_IRQ_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TP FIFO Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn tp_overrun_irq_en(&self) -> TP_OVERRUN_IRQ_EN_R {
        TP_OVERRUN_IRQ_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TP First Touch (Stylus DOWN) IRQ Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tp_down_irq_en(&mut self) -> TP_DOWN_IRQ_EN_W<0> {
        TP_DOWN_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 1 - TP Last Touch (Stylus UP) IRQ Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tp_up_irq_en(&mut self) -> TP_UP_IRQ_EN_W<1> {
        TP_UP_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 4 - TP FIFO Flush"]
    #[inline(always)]
    #[must_use]
    pub fn tp_fifo_flush(&mut self) -> TP_FIFO_FLUSH_W<4> {
        TP_FIFO_FLUSH_W::new(self)
    }
    #[doc = "Bit 7 - TP FIFO Data Available DRQ Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tp_data_erq_en(&mut self) -> TP_DATA_ERQ_EN_W<7> {
        TP_DATA_ERQ_EN_W::new(self)
    }
    #[doc = "Bits 8:12 - TP FIFO Data Available Trigger Level"]
    #[inline(always)]
    #[must_use]
    pub fn tp_fifo_trig_level(&mut self) -> TP_FIFO_TRIG_LEVEL_W<8> {
        TP_FIFO_TRIG_LEVEL_W::new(self)
    }
    #[doc = "Bit 13 - TP FIFO XY Data Interchange Function Select"]
    #[inline(always)]
    #[must_use]
    pub fn tp_data_xy_change(&mut self) -> TP_DATA_XY_CHANGE_W<13> {
        TP_DATA_XY_CHANGE_W::new(self)
    }
    #[doc = "Bit 16 - TP FIFO Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tp_data_irq_en(&mut self) -> TP_DATA_IRQ_EN_W<16> {
        TP_DATA_IRQ_EN_W::new(self)
    }
    #[doc = "Bit 17 - TP FIFO Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tp_overrun_irq_en(&mut self) -> TP_OVERRUN_IRQ_EN_W<17> {
        TP_OVERRUN_IRQ_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TP Interrupt FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tp_int_fifo_ctrl](index.html) module"]
pub struct TP_INT_FIFO_CTRL_SPEC;
impl crate::RegisterSpec for TP_INT_FIFO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tp_int_fifo_ctrl::R](R) reader structure"]
impl crate::Readable for TP_INT_FIFO_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tp_int_fifo_ctrl::W](W) writer structure"]
impl crate::Writable for TP_INT_FIFO_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets tp_int_fifo_ctrl to value 0"]
impl crate::Resettable for TP_INT_FIFO_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
