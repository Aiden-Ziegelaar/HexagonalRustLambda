import axios from 'axios';
import { assert, expect } from 'chai';
import { faker } from '@faker-js/faker';

describe('Create Product', function () {
    it('should create a product', async function () {
        //arrange
        let product = {
            product_name: faker.commerce.productName(),
            description: faker.commerce.productDescription(),
            price_cents: Number(faker.commerce.price({
                dec: 0
            }))
        }

        //act
        let res = await axios.post(`${process.env.INF_API_ENDPOINT}main/product`, product)
        
        //assert
        assert.equal(res.status, 201)
        expect(res.data).to.include(product)
    })
});