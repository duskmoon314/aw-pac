#[doc = "Register `pe_cfg2` reader"]
pub type R = crate::R<PE_CFG2_SPEC>;
#[doc = "Register `pe_cfg2` writer"]
pub type W = crate::W<PE_CFG2_SPEC>;
#[doc = "Field `pe16_select` reader - PE16 Select"]
pub type PE16_SELECT_R = crate::FieldReader<PE16_SELECT_A>;
#[doc = "PE16 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PE16_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    TWI3_SCK = 2,
    #[doc = "4: `100`"]
    PWM7 = 4,
    #[doc = "6: `110`"]
    DMIC_DATA0 = 6,
    #[doc = "14: `1110`"]
    PE_EINT16 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    D_JTAG_DO = 3,
    #[doc = "5: `101`"]
    I2S0_BCLK = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PE16_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PE16_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PE16_SELECT_A {
    type Ux = u8;
}
impl PE16_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PE16_SELECT_A> {
        match self.bits {
            0 => Some(PE16_SELECT_A::INPUT),
            2 => Some(PE16_SELECT_A::TWI3_SCK),
            4 => Some(PE16_SELECT_A::PWM7),
            6 => Some(PE16_SELECT_A::DMIC_DATA0),
            14 => Some(PE16_SELECT_A::PE_EINT16),
            1 => Some(PE16_SELECT_A::OUTPUT),
            3 => Some(PE16_SELECT_A::D_JTAG_DO),
            5 => Some(PE16_SELECT_A::I2S0_BCLK),
            15 => Some(PE16_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PE16_SELECT_A::INPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_twi3_sck(&self) -> bool {
        *self == PE16_SELECT_A::TWI3_SCK
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_pwm7(&self) -> bool {
        *self == PE16_SELECT_A::PWM7
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_dmic_data0(&self) -> bool {
        *self == PE16_SELECT_A::DMIC_DATA0
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pe_eint16(&self) -> bool {
        *self == PE16_SELECT_A::PE_EINT16
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PE16_SELECT_A::OUTPUT
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_d_jtag_do(&self) -> bool {
        *self == PE16_SELECT_A::D_JTAG_DO
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_i2s0_bclk(&self) -> bool {
        *self == PE16_SELECT_A::I2S0_BCLK
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PE16_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pe16_select` writer - PE16 Select"]
pub type PE16_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PE16_SELECT_A>;
impl<'a, REG> PE16_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PE16_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn twi3_sck(self) -> &'a mut crate::W<REG> {
        self.variant(PE16_SELECT_A::TWI3_SCK)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn pwm7(self) -> &'a mut crate::W<REG> {
        self.variant(PE16_SELECT_A::PWM7)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn dmic_data0(self) -> &'a mut crate::W<REG> {
        self.variant(PE16_SELECT_A::DMIC_DATA0)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pe_eint16(self) -> &'a mut crate::W<REG> {
        self.variant(PE16_SELECT_A::PE_EINT16)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PE16_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn d_jtag_do(self) -> &'a mut crate::W<REG> {
        self.variant(PE16_SELECT_A::D_JTAG_DO)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn i2s0_bclk(self) -> &'a mut crate::W<REG> {
        self.variant(PE16_SELECT_A::I2S0_BCLK)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PE16_SELECT_A::IO_DISABLE)
    }
}
#[doc = "Field `pe17_select` reader - PE17 Select"]
pub type PE17_SELECT_R = crate::FieldReader<PE17_SELECT_A>;
#[doc = "PE17 Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PE17_SELECT_A {
    #[doc = "0: `0`"]
    INPUT = 0,
    #[doc = "2: `10`"]
    TWI3_SDA = 2,
    #[doc = "4: `100`"]
    IR_TX = 4,
    #[doc = "6: `110`"]
    DMIC_CLK = 6,
    #[doc = "14: `1110`"]
    PE_EINT17 = 14,
    #[doc = "1: `1`"]
    OUTPUT = 1,
    #[doc = "3: `11`"]
    D_JTAG_CK = 3,
    #[doc = "5: `101`"]
    I2S0_MCLK = 5,
    #[doc = "15: `1111`"]
    IO_DISABLE = 15,
}
impl From<PE17_SELECT_A> for u8 {
    #[inline(always)]
    fn from(variant: PE17_SELECT_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PE17_SELECT_A {
    type Ux = u8;
}
impl PE17_SELECT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PE17_SELECT_A> {
        match self.bits {
            0 => Some(PE17_SELECT_A::INPUT),
            2 => Some(PE17_SELECT_A::TWI3_SDA),
            4 => Some(PE17_SELECT_A::IR_TX),
            6 => Some(PE17_SELECT_A::DMIC_CLK),
            14 => Some(PE17_SELECT_A::PE_EINT17),
            1 => Some(PE17_SELECT_A::OUTPUT),
            3 => Some(PE17_SELECT_A::D_JTAG_CK),
            5 => Some(PE17_SELECT_A::I2S0_MCLK),
            15 => Some(PE17_SELECT_A::IO_DISABLE),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_input(&self) -> bool {
        *self == PE17_SELECT_A::INPUT
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn is_twi3_sda(&self) -> bool {
        *self == PE17_SELECT_A::TWI3_SDA
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn is_ir_tx(&self) -> bool {
        *self == PE17_SELECT_A::IR_TX
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn is_dmic_clk(&self) -> bool {
        *self == PE17_SELECT_A::DMIC_CLK
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn is_pe_eint17(&self) -> bool {
        *self == PE17_SELECT_A::PE_EINT17
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PE17_SELECT_A::OUTPUT
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn is_d_jtag_ck(&self) -> bool {
        *self == PE17_SELECT_A::D_JTAG_CK
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn is_i2s0_mclk(&self) -> bool {
        *self == PE17_SELECT_A::I2S0_MCLK
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn is_io_disable(&self) -> bool {
        *self == PE17_SELECT_A::IO_DISABLE
    }
}
#[doc = "Field `pe17_select` writer - PE17 Select"]
pub type PE17_SELECT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PE17_SELECT_A>;
impl<'a, REG> PE17_SELECT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn input(self) -> &'a mut crate::W<REG> {
        self.variant(PE17_SELECT_A::INPUT)
    }
    #[doc = "`10`"]
    #[inline(always)]
    pub fn twi3_sda(self) -> &'a mut crate::W<REG> {
        self.variant(PE17_SELECT_A::TWI3_SDA)
    }
    #[doc = "`100`"]
    #[inline(always)]
    pub fn ir_tx(self) -> &'a mut crate::W<REG> {
        self.variant(PE17_SELECT_A::IR_TX)
    }
    #[doc = "`110`"]
    #[inline(always)]
    pub fn dmic_clk(self) -> &'a mut crate::W<REG> {
        self.variant(PE17_SELECT_A::DMIC_CLK)
    }
    #[doc = "`1110`"]
    #[inline(always)]
    pub fn pe_eint17(self) -> &'a mut crate::W<REG> {
        self.variant(PE17_SELECT_A::PE_EINT17)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn output(self) -> &'a mut crate::W<REG> {
        self.variant(PE17_SELECT_A::OUTPUT)
    }
    #[doc = "`11`"]
    #[inline(always)]
    pub fn d_jtag_ck(self) -> &'a mut crate::W<REG> {
        self.variant(PE17_SELECT_A::D_JTAG_CK)
    }
    #[doc = "`101`"]
    #[inline(always)]
    pub fn i2s0_mclk(self) -> &'a mut crate::W<REG> {
        self.variant(PE17_SELECT_A::I2S0_MCLK)
    }
    #[doc = "`1111`"]
    #[inline(always)]
    pub fn io_disable(self) -> &'a mut crate::W<REG> {
        self.variant(PE17_SELECT_A::IO_DISABLE)
    }
}
impl R {
    #[doc = "Bits 0:3 - PE16 Select"]
    #[inline(always)]
    pub fn pe16_select(&self) -> PE16_SELECT_R {
        PE16_SELECT_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PE17 Select"]
    #[inline(always)]
    pub fn pe17_select(&self) -> PE17_SELECT_R {
        PE17_SELECT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PE16 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pe16_select(&mut self) -> PE16_SELECT_W<PE_CFG2_SPEC> {
        PE16_SELECT_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - PE17 Select"]
    #[inline(always)]
    #[must_use]
    pub fn pe17_select(&mut self) -> PE17_SELECT_W<PE_CFG2_SPEC> {
        PE17_SELECT_W::new(self, 4)
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
#[doc = "PE Configure Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pe_cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pe_cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PE_CFG2_SPEC;
impl crate::RegisterSpec for PE_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pe_cfg2::R`](R) reader structure"]
impl crate::Readable for PE_CFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pe_cfg2::W`](W) writer structure"]
impl crate::Writable for PE_CFG2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets pe_cfg2 to value 0"]
impl crate::Resettable for PE_CFG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
