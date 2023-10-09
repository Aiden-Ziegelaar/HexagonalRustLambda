import axios from 'axios';
import { assert, expect } from 'chai';
import { faker } from '@faker-js/faker';

describe('Add Product to Cart', function () {
    it('should add a product to a users cart', async function () {
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
        let res = await axios.post(`${process.env.INF_API_ENDPOINT}main/cart`, cart_item).catch(err => {
            console.log(err)
        })

        //assert
        assert.equal(res.status, 201)
        expect(res.data).to.include(cart_item)
    })
});