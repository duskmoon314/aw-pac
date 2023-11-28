#[doc = "Register `lradc_ints` reader"]
pub type R = crate::R<LRADC_INTS_SPEC>;
#[doc = "Register `lradc_ints` writer"]
pub type W = crate::W<LRADC_INTS_SPEC>;
#[doc = "Field `adc0_data_pending` reader - ADC0 Data Pending"]
pub type ADC0_DATA_PENDING_R = crate::BitReader<ADC0_DATA_PENDING_A>;
#[doc = "ADC0 Data Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC0_DATA_PENDING_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<ADC0_DATA_PENDING_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0_DATA_PENDING_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC0_DATA_PENDING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC0_DATA_PENDING_A {
        match self.bits {
            false => ADC0_DATA_PENDING_A::NO_PENDING,
            true => ADC0_DATA_PENDING_A::PENDING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == ADC0_DATA_PENDING_A::NO_PENDING
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ADC0_DATA_PENDING_A::PENDING
    }
}
#[doc = "Field `adc0_data_pending` writer - ADC0 Data Pending"]
pub type ADC0_DATA_PENDING_W<'a, REG> = crate::BitWriter1C<'a, REG, ADC0_DATA_PENDING_A>;
impl<'a, REG> ADC0_DATA_PENDING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut crate::W<REG> {
        self.variant(ADC0_DATA_PENDING_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(ADC0_DATA_PENDING_A::PENDING)
    }
}
#[doc = "Field `adc0_keydown_pending` reader - ADC0 Key Down Pending"]
pub type ADC0_KEYDOWN_PENDING_R = crate::BitReader<ADC0_KEYDOWN_PENDING_A>;
#[doc = "ADC0 Key Down Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC0_KEYDOWN_PENDING_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<ADC0_KEYDOWN_PENDING_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0_KEYDOWN_PENDING_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC0_KEYDOWN_PENDING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC0_KEYDOWN_PENDING_A {
        match self.bits {
            false => ADC0_KEYDOWN_PENDING_A::NO_PENDING,
            true => ADC0_KEYDOWN_PENDING_A::PENDING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == ADC0_KEYDOWN_PENDING_A::NO_PENDING
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ADC0_KEYDOWN_PENDING_A::PENDING
    }
}
#[doc = "Field `adc0_keydown_pending` writer - ADC0 Key Down Pending"]
pub type ADC0_KEYDOWN_PENDING_W<'a, REG> = crate::BitWriter1C<'a, REG, ADC0_KEYDOWN_PENDING_A>;
impl<'a, REG> ADC0_KEYDOWN_PENDING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut crate::W<REG> {
        self.variant(ADC0_KEYDOWN_PENDING_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(ADC0_KEYDOWN_PENDING_A::PENDING)
    }
}
#[doc = "Field `adc0_hold_pending` reader - ADC0 Hold Key Pending"]
pub type ADC0_HOLD_PENDING_R = crate::BitReader<ADC0_HOLD_PENDING_A>;
#[doc = "ADC0 Hold Key Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC0_HOLD_PENDING_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<ADC0_HOLD_PENDING_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0_HOLD_PENDING_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC0_HOLD_PENDING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC0_HOLD_PENDING_A {
        match self.bits {
            false => ADC0_HOLD_PENDING_A::NO_PENDING,
            true => ADC0_HOLD_PENDING_A::PENDING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == ADC0_HOLD_PENDING_A::NO_PENDING
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ADC0_HOLD_PENDING_A::PENDING
    }
}
#[doc = "Field `adc0_hold_pending` writer - ADC0 Hold Key Pending"]
pub type ADC0_HOLD_PENDING_W<'a, REG> = crate::BitWriter1C<'a, REG, ADC0_HOLD_PENDING_A>;
impl<'a, REG> ADC0_HOLD_PENDING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut crate::W<REG> {
        self.variant(ADC0_HOLD_PENDING_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(ADC0_HOLD_PENDING_A::PENDING)
    }
}
#[doc = "Field `adc0_alrdy_hold_pending` reader - ADC0 Already Hold Key Pending"]
pub type ADC0_ALRDY_HOLD_PENDING_R = crate::BitReader<ADC0_ALRDY_HOLD_PENDING_A>;
#[doc = "ADC0 Already Hold Key Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC0_ALRDY_HOLD_PENDING_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<ADC0_ALRDY_HOLD_PENDING_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0_ALRDY_HOLD_PENDING_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC0_ALRDY_HOLD_PENDING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC0_ALRDY_HOLD_PENDING_A {
        match self.bits {
            false => ADC0_ALRDY_HOLD_PENDING_A::NO_PENDING,
            true => ADC0_ALRDY_HOLD_PENDING_A::PENDING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == ADC0_ALRDY_HOLD_PENDING_A::NO_PENDING
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ADC0_ALRDY_HOLD_PENDING_A::PENDING
    }
}
#[doc = "Field `adc0_alrdy_hold_pending` writer - ADC0 Already Hold Key Pending"]
pub type ADC0_ALRDY_HOLD_PENDING_W<'a, REG> =
    crate::BitWriter1C<'a, REG, ADC0_ALRDY_HOLD_PENDING_A>;
impl<'a, REG> ADC0_ALRDY_HOLD_PENDING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut crate::W<REG> {
        self.variant(ADC0_ALRDY_HOLD_PENDING_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(ADC0_ALRDY_HOLD_PENDING_A::PENDING)
    }
}
#[doc = "Field `adc0_keyup_pending` reader - ADC0 Key Up Pending"]
pub type ADC0_KEYUP_PENDING_R = crate::BitReader<ADC0_KEYUP_PENDING_A>;
#[doc = "ADC0 Key Up Pending\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADC0_KEYUP_PENDING_A {
    #[doc = "0: `0`"]
    NO_PENDING = 0,
    #[doc = "1: `1`"]
    PENDING = 1,
}
impl From<ADC0_KEYUP_PENDING_A> for bool {
    #[inline(always)]
    fn from(variant: ADC0_KEYUP_PENDING_A) -> Self {
        variant as u8 != 0
    }
}
impl ADC0_KEYUP_PENDING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ADC0_KEYUP_PENDING_A {
        match self.bits {
            false => ADC0_KEYUP_PENDING_A::NO_PENDING,
            true => ADC0_KEYUP_PENDING_A::PENDING,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_no_pending(&self) -> bool {
        *self == ADC0_KEYUP_PENDING_A::NO_PENDING
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_pending(&self) -> bool {
        *self == ADC0_KEYUP_PENDING_A::PENDING
    }
}
#[doc = "Field `adc0_keyup_pending` writer - ADC0 Key Up Pending"]
pub type ADC0_KEYUP_PENDING_W<'a, REG> = crate::BitWriter1C<'a, REG, ADC0_KEYUP_PENDING_A>;
impl<'a, REG> ADC0_KEYUP_PENDING_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn no_pending(self) -> &'a mut crate::W<REG> {
        self.variant(ADC0_KEYUP_PENDING_A::NO_PENDING)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn pending(self) -> &'a mut crate::W<REG> {
        self.variant(ADC0_KEYUP_PENDING_A::PENDING)
    }
}
impl R {
    #[doc = "Bit 0 - ADC0 Data Pending"]
    #[inline(always)]
    pub fn adc0_data_pending(&self) -> ADC0_DATA_PENDING_R {
        ADC0_DATA_PENDING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC0 Key Down Pending"]
    #[inline(always)]
    pub fn adc0_keydown_pending(&self) -> ADC0_KEYDOWN_PENDING_R {
        ADC0_KEYDOWN_PENDING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC0 Hold Key Pending"]
    #[inline(always)]
    pub fn adc0_hold_pending(&self) -> ADC0_HOLD_PENDING_R {
        ADC0_HOLD_PENDING_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC0 Already Hold Key Pending"]
    #[inline(always)]
    pub fn adc0_alrdy_hold_pending(&self) -> ADC0_ALRDY_HOLD_PENDING_R {
        ADC0_ALRDY_HOLD_PENDING_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC0 Key Up Pending"]
    #[inline(always)]
    pub fn adc0_keyup_pending(&self) -> ADC0_KEYUP_PENDING_R {
        ADC0_KEYUP_PENDING_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC0 Data Pending"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_data_pending(&mut self) -> ADC0_DATA_PENDING_W<LRADC_INTS_SPEC> {
        ADC0_DATA_PENDING_W::new(self, 0)
    }
    #[doc = "Bit 1 - ADC0 Key Down Pending"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_keydown_pending(&mut self) -> ADC0_KEYDOWN_PENDING_W<LRADC_INTS_SPEC> {
        ADC0_KEYDOWN_PENDING_W::new(self, 1)
    }
    #[doc = "Bit 2 - ADC0 Hold Key Pending"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_hold_pending(&mut self) -> ADC0_HOLD_PENDING_W<LRADC_INTS_SPEC> {
        ADC0_HOLD_PENDING_W::new(self, 2)
    }
    #[doc = "Bit 3 - ADC0 Already Hold Key Pending"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_alrdy_hold_pending(&mut self) -> ADC0_ALRDY_HOLD_PENDING_W<LRADC_INTS_SPEC> {
        ADC0_ALRDY_HOLD_PENDING_W::new(self, 3)
    }
    #[doc = "Bit 4 - ADC0 Key Up Pending"]
    #[inline(always)]
    #[must_use]
    pub fn adc0_keyup_pending(&mut self) -> ADC0_KEYUP_PENDING_W<LRADC_INTS_SPEC> {
        ADC0_KEYUP_PENDING_W::new(self, 4)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "LRADC Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lradc_ints::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lradc_ints::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LRADC_INTS_SPEC;
impl crate::RegisterSpec for LRADC_INTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lradc_ints::R`](R) reader structure"]
impl crate::Readable for LRADC_INTS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lradc_ints::W`](W) writer structure"]
impl crate::Writable for LRADC_INTS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x1f;
}
#[doc = "`reset()` method sets lradc_ints to value 0"]
impl crate::Resettable for LRADC_INTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
