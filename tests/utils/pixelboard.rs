use super::other::*;
use core::marker::PhantomData;
use gear_lib::non_fungible_token::token::TokenMetadata;
use gstd::ActorId;
use gtest::{Program as InnerProgram, System};

pub struct NFTPixelboard<'a>(InnerProgram<'a>);

impl Program for NFTPixelboard<'_> {
    fn inner_program(&self) -> &InnerProgram {
        &self.0
    }
}

impl<'a> NFTPixelboard<'a> {
    pub fn initialize(
        system: &'a System,
        ft_program: ActorId,
        nft_program: ActorId,
    ) -> NFTPixelboardInit {
        Self::initialize_custom(
            system,
            InitNFTPixelboard {
                ft_program,
                min_block_side_length: 1,
                nft_program,
                owner: OWNER.into(),
                painting: vec![0; 100],
                pixel_price: MAX_PIXEL_PRICE,
                resale_commission_percentage: 100,
                resolution: (10, 10).into(),
            },
        )
    }

    pub fn initialize_custom(system: &'a System, config: InitNFTPixelboard) -> NFTPixelboardInit {
        let program = InnerProgram::current(system);

        let failed = program.send(FOREIGN_USER, config).main_failed();

        NFTPixelboardInit(program, failed)
    }

    pub fn meta_state(&self) -> NFTPixelboardMetaState {
        NFTPixelboardMetaState(&self.0)
    }

    pub fn mint(
        &self,
        from: u64,
        painting: Vec<Color>,
        rectangle: Rectangle,
    ) -> Action<impl FnOnce(u128) -> NFTPixelboardEvent, u128, NFTPixelboardEvent> {
        self.mint_with_metadata(from, painting, rectangle, Default::default())
    }

    pub fn mint_with_metadata(
        &self,
        from: u64,
        painting: Vec<Color>,
        rectangle: Rectangle,
        token_metadata: TokenMetadata,
    ) -> Action<impl FnOnce(u128) -> NFTPixelboardEvent, u128, NFTPixelboardEvent> {
        Action(
            self.0.send(
                from,
                NFTPixelboardAction::Mint {
                    painting,
                    rectangle,
                    token_metadata,
                },
            ),
            |value| NFTPixelboardEvent::Minted(value.into()),
            PhantomData,
        )
    }

    pub fn put_up_for_sale(
        &self,
        from: u64,
        token_id: u128,
        pixel_price: u128,
    ) -> Action<impl FnOnce(u128) -> NFTPixelboardEvent, u128, NFTPixelboardEvent> {
        Action(
            self.0.send(
                from,
                NFTPixelboardAction::PutUpForSale {
                    token_id: token_id.into(),
                    pixel_price,
                },
            ),
            |token_id| NFTPixelboardEvent::ForSale(token_id.into()),
            PhantomData,
        )
    }

    pub fn buy(
        &self,
        from: u64,
        token_id: u128,
    ) -> Action<impl FnOnce(u128) -> NFTPixelboardEvent, u128, NFTPixelboardEvent> {
        Action(
            self.0.send(from, NFTPixelboardAction::Buy(token_id.into())),
            |token_id| NFTPixelboardEvent::Bought(token_id.into()),
            PhantomData,
        )
    }

    pub fn paint(
        &self,
        from: u64,
        token_id: u128,
        painting: Vec<Color>,
    ) -> Action<impl FnOnce(u128) -> NFTPixelboardEvent, u128, NFTPixelboardEvent> {
        Action(
            self.0.send(
                from,
                NFTPixelboardAction::Paint {
                    token_id: token_id.into(),
                    painting,
                },
            ),
            |token_id| NFTPixelboardEvent::Painted(token_id.into()),
            PhantomData,
        )
    }
}

pub struct NFTPixelboardInit<'a>(InnerProgram<'a>, bool);

impl<'a> NFTPixelboardInit<'a> {
    #[track_caller]
    pub fn failed(self) {
        assert!(self.1)
    }

    #[track_caller]
    pub fn succeed(self) -> NFTPixelboard<'a> {
        assert!(!self.1);
        NFTPixelboard(self.0)
    }
}

pub struct NFTPixelboardMetaState<'a>(&'a InnerProgram<'a>);

impl NFTPixelboardMetaState<'_> {
    pub fn ft_program(self) -> MetaStateReply<ActorId> {
        if let NFTPixelboardStateReply::FTProgram(reply) =
            self.0.meta_state(NFTPixelboardState::FTProgram).unwrap()
        {
            MetaStateReply(reply)
        } else {
            unreachable!();
        }
    }

    pub fn nft_program(self) -> MetaStateReply<ActorId> {
        if let NFTPixelboardStateReply::NFTProgram(reply) =
            self.0.meta_state(NFTPixelboardState::NFTProgram).unwrap()
        {
            MetaStateReply(reply)
        } else {
            unreachable!();
        }
    }

    pub fn min_block_side_length(self) -> MetaStateReply<MinBlockSideLength> {
        if let NFTPixelboardStateReply::MinBlockSideLength(reply) = self
            .0
            .meta_state(NFTPixelboardState::MinBlockSideLength)
            .unwrap()
        {
            MetaStateReply(reply)
        } else {
            unreachable!();
        }
    }

    pub fn painting(self) -> MetaStateReply<Vec<Color>> {
        if let NFTPixelboardStateReply::Painting(reply) =
            self.0.meta_state(NFTPixelboardState::Painting).unwrap()
        {
            MetaStateReply(reply)
        } else {
            unreachable!();
        }
    }

    pub fn pixel_info(self, coordinates: Coordinates) -> MetaStateReply<Token> {
        if let NFTPixelboardStateReply::PixelInfo(reply) = self
            .0
            .meta_state(NFTPixelboardState::PixelInfo(coordinates))
            .unwrap()
        {
            MetaStateReply(reply)
        } else {
            unreachable!();
        }
    }

    pub fn pixel_price(self) -> MetaStateReply<u128> {
        if let NFTPixelboardStateReply::PixelPrice(reply) =
            self.0.meta_state(NFTPixelboardState::PixelPrice).unwrap()
        {
            MetaStateReply(reply)
        } else {
            unreachable!();
        }
    }

    pub fn resale_commission_percentage(self) -> MetaStateReply<u8> {
        if let NFTPixelboardStateReply::CommissionPercentage(reply) = self
            .0
            .meta_state(NFTPixelboardState::CommissionPercentage)
            .unwrap()
        {
            MetaStateReply(reply)
        } else {
            unreachable!();
        }
    }

    pub fn resolution(self) -> MetaStateReply<Resolution> {
        if let NFTPixelboardStateReply::Resolution(reply) =
            self.0.meta_state(NFTPixelboardState::Resolution).unwrap()
        {
            MetaStateReply(reply)
        } else {
            unreachable!();
        }
    }

    pub fn token_info(self, token_id: u128) -> MetaStateReply<Token> {
        if let NFTPixelboardStateReply::TokenInfo(reply) = self
            .0
            .meta_state(NFTPixelboardState::TokenInfo(token_id.into()))
            .unwrap()
        {
            MetaStateReply(reply)
        } else {
            unreachable!();
        }
    }
}
