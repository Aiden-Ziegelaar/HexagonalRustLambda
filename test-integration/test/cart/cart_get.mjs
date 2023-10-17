import axios from 'axios';
import { assert, expect } from 'chai';
import { faker } from '@faker-js/faker';

describe('Get Cart', function () {
    it('should add a product to a users cart then retrieve the cart', async function () {
        //arrange
        let cart_item = {
            product_id: faker.string.uuid(),
            user_id: faker.internet.email(),
            quantity: faker.number.int({
                min: 1, 
                max: 10
            })
        }

        //act
        await axios.post(`${process.env.INF_API_ENDPOINT}main/cart/item`, cart_item).catch(err => {
            console.log(err)
        })

        let res = await axios.get(`${process.env.INF_API_ENDPOINT}main/cart`, {
            params: {
                id: cart_item.user_id
            }
        })

        cart_item.user_id = cart_item.user_id.toLowerCase();

        //assert
        assert.equal(res.status, 200)
        expect(res.data[0]).to.include(cart_item)
    })

    it('should add multiple products to a users cart then retrieve the cart', async function () {
        //arrange
        let user_id = faker.internet.email();

        let cart_items = Array(10).fill().map(() => {
                return {
                    product_id: faker.string.uuid(),
                    user_id,
                    quantity: faker.number.int({
                        min: 1, 
                        max: 10
                    })
                }
            })

        //act
        let res_post_promises = cart_items.map(
            cart_item => axios.post(`${process.env.INF_API_ENDPOINT}main/cart/item`, cart_item)
        )
        await Promise.all(res_post_promises);

        let res = await axios.get(`${process.env.INF_API_ENDPOINT}main/cart`, {
            params: {
                id: user_id
            }
        })

        //assert
        assert.equal(res.status, 200);
        assert.equal(res.data.length, 10);
    })

    it('should return an empty array for no cart', async function () {
        //arrange
        let user_id = faker.internet.email();

        //act
        let res = await axios.get(`${process.env.INF_API_ENDPOINT}main/cart`, {
            params: {
                id: user_id
            }
        })

        //assert
        assert.equal(res.status, 200);
        assert.equal(res.data.length, 0);
    })
});