import axios from 'axios';
import { assert, expect } from 'chai';
import { faker } from '@faker-js/faker';

describe('Get Product', function () {
    it('should get a created product', async function () {
        //arrange
        let product = {
            product_name: faker.commerce.productName(),
            description: faker.commerce.productDescription(),
            price_cents: Number(faker.commerce.price({
                dec: 0
            }))
        }

        //act
        let res_post = await axios.post(`${process.env.INF_API_ENDPOINT}main/product`, product)

        let res_get = await axios.get(`${process.env.INF_API_ENDPOINT}main/product`,
            {
                params: {
                    id: res_post.data.id
                }
            }
        )

        //assert
        assert.equal(res_get.status, 200)
        expect(res_get.data).to.include(product)
    })

    it('should not get a nonexistant product', async function () {
        //arrange
        let id = faker.string.uuid()

        //act
        let res_get = await axios.get(`${process.env.INF_API_ENDPOINT}main/product`,
            {
                params: {
                    id
                },
                validateStatus: () => true
            }
        )

        //assert
        assert.equal(res_get.status, 404)
    })
});