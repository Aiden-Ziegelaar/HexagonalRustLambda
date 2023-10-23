import axios from 'axios';
import { assert, expect } from 'chai';
import { faker } from '@faker-js/faker';

describe('Update Product', function () {
    it('should update a product', async function () {
        //arrange
        let product = {
            product_name: faker.commerce.productName(),
            description: faker.commerce.productDescription(),
            price_cents: Number(faker.commerce.price({
                dec: 0
            }))
        }

        let product_patch = {
            product_name: faker.commerce.productName(),
            description: faker.commerce.productDescription(),
            price_cents: Number(faker.commerce.price({
                dec: 0
            }))
        }

        //act
        let post_res = await axios.post(`${process.env.INF_API_ENDPOINT}main/product`, product)

        let res = await axios.put(`${process.env.INF_API_ENDPOINT}main/product/${post_res.data.id}`, 
            product_patch
        )

        //assert
        assert.equal(res.status, 200)
        expect(res.data).to.include(product_patch)
        expect(res.data).to.include({
            id: post_res.data.id
        })
    })

    it('should fail to update a product with no fields', async function () {
        //arrange
        let product = {
            product_name: faker.commerce.productName(),
            description: faker.commerce.productDescription(),
            price_cents: Number(faker.commerce.price({
                dec: 0
            }))
        }

        let product_patch = {
        }

        //act
        let post_res = await axios.post(`${process.env.INF_API_ENDPOINT}main/product`, product)

        let res = await axios.put(`${process.env.INF_API_ENDPOINT}main/product/${post_res.data.id}}`, 
            product_patch,
            {
                validateStatus: () => true
            }
        )

        //assert
        assert.equal(res.status, 400)
    })

    it('should fail to update a product that doesn\'t exist', async function () {
        //arrange
        let uuid = faker.string.uuid()
    
        let product_patch = {
            product_name: faker.commerce.productName(),
            description: faker.commerce.productDescription(),
            price_cents: Number(faker.commerce.price({
                dec: 0
            }))
        }
        
        //act
        let res = await axios.put(`${process.env.INF_API_ENDPOINT}main/product/${uuid}`, 
            product_patch,
            {
                validateStatus: () => true
            }
        )

        //assert
        assert.equal(res.status, 404)
    })
});